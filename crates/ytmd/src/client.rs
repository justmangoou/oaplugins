use std::sync::{
	LazyLock,
	atomic::{AtomicBool, AtomicI16, Ordering},
};

use openaction::{Action, Instance, OpenActionResult, set_global_settings, visible_instances};
use tokio::sync::{Mutex, RwLock};
use ytmd_companion::{
	Client, ClientSettings,
	models::{RepeatMode, TrackState, request::CommandRequest, response::WebsocketEvent},
};

use crate::GLOBAL_SETTINGS;

#[derive(Default, Clone)]
pub struct PlayerWrapper {
	pub track_state: TrackState,
	pub muted: bool,
	pub volume: u32,
	pub repeat_mode: RepeatMode,
}

pub static YTMD_CLIENT: LazyLock<Mutex<Option<Client>>> = LazyLock::new(|| Mutex::new(None));

pub static YTMD_PLAYER: LazyLock<RwLock<PlayerWrapper>> =
	LazyLock::new(|| RwLock::new(PlayerWrapper::default()));

static RECONNECTING: AtomicBool = AtomicBool::new(false);
static VOLUME_WATCHER_RUNNING: AtomicBool = AtomicBool::new(false);
pub static VOLUME_CHANGE_ACCUMULATOR: AtomicI16 = AtomicI16::new(0);

pub(crate) async fn send_command(
	instance: &Instance,
	command: &CommandRequest,
) -> OpenActionResult<()> {
	let client_lock = YTMD_CLIENT.lock().await;

	let client = match client_lock.as_ref() {
		Some(c) => c,
		None => {
			log::error!("YouTube Music client is not connected");
			instance.show_alert().await?;
			return Ok(());
		}
	};

	if let Err(e) = client.send_command(command).await {
		log::error!("Failed to send command {:?}: {}", command, e);
		instance.show_alert().await?;
	}

	Ok(())
}

pub(crate) async fn update_error(error: Option<&str>) {
	let current_guard = GLOBAL_SETTINGS.read().await;

	if current_guard.error.as_deref() == error {
		return;
	}

	let mut updated_settings = (*current_guard).clone();
	updated_settings.error = error.map(|s| s.to_owned());

	if let Err(e) = set_global_settings(&updated_settings).await {
		log::error!("Failed to save error to global settings: {}", e);
	}

	drop(current_guard);
}

pub(crate) fn schedule_reconnect() {
	if RECONNECTING.swap(true, Ordering::SeqCst) {
		return;
	}

	tokio::spawn(async {
		while RECONNECTING.load(Ordering::SeqCst) {
			if let Err(e) = reinitialize().await {
				log::error!("Reconnect attempt failed: {}", e);
			}
			tokio::time::sleep(std::time::Duration::from_secs(5)).await;
		}
	});
}

async fn reinitialize() -> Result<(), String> {
	let settings = (*GLOBAL_SETTINGS.read().await).clone();

	let client = setup_client(settings.client_settings.clone()).await?;
	let mut new_settings = settings;
	new_settings.client_settings.token = client.settings.token.clone();

	*YTMD_CLIENT.lock().await = Some(client);
	RECONNECTING.store(false, Ordering::SeqCst);
	update_error(None).await;

	openaction::set_global_settings(&new_settings)
		.await
		.map_err(|e| format!("Failed to save global settings: {}", e))?;

	log::info!("YTMD authentication successful!");

	Ok(())
}

async fn setup_client(client_settings: ClientSettings) -> Result<Client, String> {
	let mut client = Client::new(client_settings);

	client
		.connect(false)
		.await
		.map_err(|e| format!("Failed to connect to YTMD: {}", e))?;

	client
		.setup_event_handler(handle_ws_event)
		.map_err(|e| format!("Failed to set up event handler for YTMD client: {}", e))?;

	volume_change_watcher();

	Ok(client)
}

async fn handle_ws_event(item: WebsocketEvent) {
	async fn call_did_receive_settings(action_uuid: &'static str) {
		for instance in visible_instances(action_uuid).await {
			if let Err(e) = instance.get_settings().await {
				log::error!(
					"Failed to call did_receive_settings for action {}: {}",
					action_uuid,
					e
				);
			}
		}
	}

	match item {
		WebsocketEvent::StateUpdate(state) => {
			*YTMD_PLAYER.write().await = PlayerWrapper {
				track_state: state.player.track_state,
				muted: state.player.muted,
				volume: state.player.volume,
				repeat_mode: state
					.player
					.queue
					.map(|q| q.repeat_mode)
					.unwrap_or_default(),
			};

			call_did_receive_settings(crate::actions::PlayPauseAction::UUID).await;
			call_did_receive_settings(crate::actions::RepeatAction::UUID).await;
		}
		WebsocketEvent::Error(error) => {
			log::error!("Received error event: {}", error);
		}
	}
}

fn volume_change_watcher() {
	if VOLUME_WATCHER_RUNNING.swap(true, Ordering::SeqCst) {
		return;
	}

	tokio::spawn(async {
		let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));

		loop {
			interval.tick().await;

			let delta = VOLUME_CHANGE_ACCUMULATOR.swap(0, Ordering::Relaxed);

			if delta == 0 {
				continue;
			}

			let current_volume = YTMD_PLAYER.read().await.volume as i16;
			let new_volume = (current_volume + delta).clamp(0, 100) as u8;

			let client_lock = YTMD_CLIENT.lock().await;
			let client = match client_lock.as_ref() {
				Some(c) => c,
				None => {
					log::error!("YouTube Music client is not connected");
					continue;
				}
			};

			if let Err(e) = client
				.send_command(&CommandRequest::SetVolume(new_volume))
				.await
			{
				log::error!("Failed to send volume change command: {}", e);
			}
		}
	});
}

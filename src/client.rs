use std::sync::{Arc, OnceLock};

use arc_swap::{ArcSwap, ArcSwapOption};
use openaction::set_global_settings;
use oaytmd_companion::{
	Client, ClientSettings,
	models::{RepeatMode, request::CommandRequest},
};

use crate::{actions::VOLUME_CHANGE_ACCUMULATOR, current_settings, ws_events::handle_ws_event};

#[derive(Default)]
pub struct PlayerWrapper {
	pub muted: bool,
	pub volume: u32,
	pub repeat_mode: RepeatMode,
}

pub fn ytmd_client() -> &'static ArcSwapOption<Client> {
	static CLIENT: OnceLock<ArcSwapOption<Client>> = OnceLock::new();
	CLIENT.get_or_init(|| ArcSwapOption::from(None))
}

pub fn ytmd_player() -> &'static ArcSwap<PlayerWrapper> {
	static PLAYER: OnceLock<ArcSwap<PlayerWrapper>> = OnceLock::new();
	PLAYER.get_or_init(|| ArcSwap::from_pointee(PlayerWrapper::default()))
}

pub(crate) async fn update_error(error: Option<&str>) {
	let current_guard = current_settings().load();

	if current_guard.error.as_deref() == error {
		return;
	}

	let mut updated_settings = (**current_guard).clone();
	updated_settings.error = error.map(|s| s.to_owned());

	if let Err(e) = set_global_settings(&updated_settings).await {
		log::error!("Failed to save error to global settings: {}", e);
	}

	current_settings().store(Arc::new(updated_settings));
}

pub(crate) async fn reinitialize() {
	let client_settings = {
		let settings = current_settings().load();
		settings.client_settings.clone()
	};

	let client = match setup_client(client_settings).await {
		Ok(client) => client,
		Err(e) => {
			log::error!("Failed to connect to YTMD: {e}");
			update_error(Some(&format!("Connection failed: {e}"))).await;
			return;
		}
	};

	log::info!("YTMD Authentication successful!");

	let token = client.settings.load().token.clone();
	let client_arc = Arc::new(client);

	ytmd_client().store(Some(client_arc.clone()));

	let current_guard = current_settings().load();
	let mut updated_settings = (**current_guard).clone();
	updated_settings.client_settings.token = token;

	if let Err(e) = openaction::set_global_settings(&updated_settings).await {
		log::error!("Failed to persist token to OpenAction store: {}", e);
	}
}

async fn setup_client(client_settings: ClientSettings) -> Result<Client, String> {
	let client = Client::new(client_settings);

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

fn volume_change_watcher() {
	tokio::spawn(async {
		let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));

		loop {
			interval.tick().await;

			let delta = {
				let mut accumulated = VOLUME_CHANGE_ACCUMULATOR.lock().await;

				let delta = *accumulated;
				*accumulated = 0;
				delta
			} as i16;

			if delta == 0 {
				continue;
			}

			let current_volume = ytmd_player().load().volume as i16;
			let new_volume = (current_volume + delta).clamp(0, 100) as u8;

			let client_lock = ytmd_client().load();
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

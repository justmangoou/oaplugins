use arc_swap::ArcSwap;
use std::sync::Arc;
use std::{ops::Deref, sync::OnceLock};

use openaction::{
	OpenActionResult, async_trait, get_global_settings, global_events, register_action, run,
};
use serde::{Deserialize, Serialize};

use crate::client::reinitialize;

mod actions;
mod client;
mod ws_events;

use actions::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GlobalSettings {
	pub client_settings: ytmd_companion_rs::ClientSettings,
	pub error: Option<String>,
}

impl Default for GlobalSettings {
	fn default() -> Self {
		Self {
			client_settings: ytmd_companion_rs::ClientSettings {
				app_id: env!("CARGO_PKG_NAME").to_owned(),
				app_name: "YTMD Controller".to_owned(),
				app_version: env!("CARGO_PKG_VERSION").to_owned(),
				host: "127.0.0.1".to_owned(),
				port: 9863,
				token: None,
			},
			error: None,
		}
	}
}

impl Deref for GlobalSettings {
	type Target = ytmd_companion_rs::ClientSettings;

	fn deref(&self) -> &Self::Target {
		&self.client_settings
	}
}

pub fn current_settings() -> &'static ArcSwap<GlobalSettings> {
	static SETTINGS: OnceLock<ArcSwap<GlobalSettings>> = OnceLock::new();
	SETTINGS.get_or_init(|| ArcSwap::from_pointee(GlobalSettings::default()))
}

pub struct GlobalEventHandler;
#[async_trait]
impl global_events::GlobalEventHandler for GlobalEventHandler {
	async fn plugin_ready(&self) -> OpenActionResult<()> {
		get_global_settings().await
	}

	async fn did_receive_global_settings(
		&self,
		event: global_events::DidReceiveGlobalSettingsEvent,
	) -> OpenActionResult<()> {
		let settings: GlobalSettings =
			serde_json::from_value(event.payload.settings).unwrap_or_default();

		let current = current_settings().load();
		let settings_changed = current.host != settings.host
			|| current.port != settings.port
			|| current.token != settings.token
			|| current.host.is_empty()
			|| current.token.is_none();
		drop(current);

		if settings_changed {
			log::info!("Global settings changed, reinitializing YTMD client");

			// Persist configuration change instantly across threads
			current_settings().store(Arc::new(settings));

			reinitialize().await;
		}

		Ok(())
	}
}

#[tokio::main]
async fn main() -> OpenActionResult<()> {
	{
		use simplelog::*;
		if let Err(error) = TermLogger::init(
			LevelFilter::Debug,
			Config::default(),
			TerminalMode::Stdout,
			ColorChoice::Never,
		) {
			eprintln!("Logger initialization failed: {}", error);
		}
	}

	global_events::set_global_event_handler(&GlobalEventHandler);
	register_action(PlayPauseAction).await;
	register_action(NextAction).await;
	register_action(PreviousAction).await;
	register_action(RepeatAction).await;
	register_action(ShuffleAction).await;
	register_action(VolumeControlAction).await;

	run(std::env::args().collect()).await
}

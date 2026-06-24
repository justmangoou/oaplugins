use std::sync::LazyLock;

use openaction::{
	OpenActionResult, async_trait, get_global_settings, global_events, register_action,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::client::schedule_reconnect;

mod actions;
mod client;

use actions::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GlobalSettings {
	pub client_settings: ytmd_companion::ClientSettings,
	pub error: Option<String>,
}

impl Default for GlobalSettings {
	fn default() -> Self {
		Self {
			client_settings: ytmd_companion::ClientSettings {
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

impl std::ops::Deref for GlobalSettings {
	type Target = ytmd_companion::ClientSettings;

	fn deref(&self) -> &Self::Target {
		&self.client_settings
	}
}

pub static GLOBAL_SETTINGS: LazyLock<RwLock<GlobalSettings>> =
	LazyLock::new(|| RwLock::new(GlobalSettings::default()));

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

		let current = GLOBAL_SETTINGS.read().await;
		let settings_changed = current.host != settings.host
			|| current.port != settings.port
			|| current.token != settings.token
			|| current.host.is_empty()
			|| current.token.is_none();
		drop(current);

		if settings_changed {
			log::info!("Global settings changed, reinitializing YTMD client");

			*GLOBAL_SETTINGS.write().await = settings;

			schedule_reconnect();
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

	openaction::run(std::env::args().collect()).await
}

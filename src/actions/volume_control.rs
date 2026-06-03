use openaction::{Action, Instance, OpenActionResult, async_trait};
use serde::{Deserialize, Serialize};
use oaytmd_companion::models::request::CommandRequest;

use crate::{actions::send_command, client::ytmd_player};

pub static VOLUME_CHANGE_ACCUMULATOR: tokio::sync::Mutex<i16> = tokio::sync::Mutex::const_new(0);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct VolumeControlActionSettings {
	pub step_size: u8,
}

impl Default for VolumeControlActionSettings {
	fn default() -> Self {
		Self { step_size: 3 }
	}
}

pub struct VolumeControlAction;

#[async_trait]
impl Action for VolumeControlAction {
	const UUID: &'static str = "justmangoou.oaytmd.volumecontrol";
	type Settings = VolumeControlActionSettings;

	async fn dial_up(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		let muted = ytmd_player().load().muted;

		if !muted {
			send_command(instance, &CommandRequest::Mute).await
		} else {
			send_command(instance, &CommandRequest::Unmute).await
		}
	}

	async fn dial_rotate(
		&self,
		_instance: &Instance,
		settings: &Self::Settings,
		ticks: i16,
		_pressed: bool,
	) -> OpenActionResult<()> {
	    let delta = ticks * settings.step_size as i16;
		let mut accumulated = VOLUME_CHANGE_ACCUMULATOR.lock().await;

		*accumulated += delta;

		Ok(())
	}
}

use std::sync::atomic::Ordering;

use openaction::{Action, Instance, OpenActionResult, async_trait};
use serde::{Deserialize, Serialize};
use ytmd_companion::models::request::CommandRequest;

use crate::client::{VOLUME_CHANGE_ACCUMULATOR, YTMD_PLAYER, send_command};

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
		let muted = YTMD_PLAYER.read().await.muted;

		if !muted {
			send_command(instance, &CommandRequest::Mute).await
		} else {
			send_command(instance, &CommandRequest::Unmute).await
		}
	}

	async fn dial_rotate(
		&self,
		instance: &Instance,
		settings: &Self::Settings,
		ticks: i16,
		_pressed: bool,
	) -> OpenActionResult<()> {
		let current_volume = YTMD_PLAYER.read().await.volume as i16;
		let delta = ticks * settings.step_size as i16;

		if (current_volume == 0 && delta < 0) || (current_volume == 100 && delta > 0) {
			instance.show_alert().await?;
			return Ok(());
		}

		VOLUME_CHANGE_ACCUMULATOR.fetch_add(delta, Ordering::Relaxed);

		Ok(())
	}
}

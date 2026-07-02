use std::sync::atomic::Ordering;

use openaction::{Action, Instance, OpenActionResult, async_trait};
use serde::{Deserialize, Serialize};
use ytmd_companion::models::{TrackState, request::CommandRequest};

use crate::client::{VOLUME_CHANGE_ACCUMULATOR, YTMD_PLAYER, send_command};

const PLAY_ICON: &str = include_str!("../../assets/encoders/play.svg");
const PAUSE_ICON: &str = include_str!("../../assets/encoders/pause.svg");

pub async fn update_feedback(instance: &Instance) {
	let player = YTMD_PLAYER.read().await;
	let icon = match player.track_state {
		TrackState::Unknown | TrackState::Buffering | TrackState::Playing => PAUSE_ICON,
		TrackState::Paused => PLAY_ICON,
	};

	let volume = if player.muted { 0 } else { player.volume };
	let volume_text = if player.muted {
		"Muted"
	} else {
		&format!("Vol: {}%", volume)
	};

	instance
		.set_feedback(&serde_json::json!({
			"icon": icon,
			"value": volume_text,
			"indicator": {
				"value": volume,
			}
		}))
		.await
		.ok();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct PlayblackVolumeSettings {
	pub step_size: u8,
}

impl Default for PlayblackVolumeSettings {
	fn default() -> Self {
		Self { step_size: 5 }
	}
}

pub struct PlayblackVolumeAction;

#[async_trait]
impl Action for PlayblackVolumeAction {
	const UUID: &'static str = "justmangoou.oaytmd.playblackvolume";
	type Settings = PlayblackVolumeSettings;

	async fn will_appear(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		update_feedback(instance).await;
		Ok(())
	}

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

	async fn touch_tap(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
		_position: (u16, u16),
		_hold: bool,
	) -> OpenActionResult<()> {
		send_command(instance, &CommandRequest::PlayPause).await
	}
}

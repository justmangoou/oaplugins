use std::collections::HashMap;

use openaction::{Action, Instance, OpenActionResult, async_trait};
use ytmd_companion::models::{TrackState, request::CommandRequest};

use crate::client::{YTMD_PLAYER, send_command};

pub struct PlayPauseAction;

async fn set_new_state(instance: &Instance, state: &TrackState) -> OpenActionResult<()> {
	let new_state = match state {
		TrackState::Unknown | TrackState::Paused => 0,
		TrackState::Buffering | TrackState::Playing => 1,
	};

	instance.set_state(new_state).await
}

#[async_trait]
impl Action for PlayPauseAction {
	const UUID: &'static str = "justmangoou.oaytmd.playpause";
	type Settings = HashMap<String, String>;

	async fn did_receive_settings(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		let player = YTMD_PLAYER.read().await;
		set_new_state(instance, &player.track_state).await
	}

	async fn will_appear(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		let player = YTMD_PLAYER.read().await;
		set_new_state(instance, &player.track_state).await
	}

	async fn key_up(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		send_command(instance, &CommandRequest::PlayPause).await
	}
}

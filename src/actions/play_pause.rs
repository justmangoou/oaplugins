use openaction::{Action, Instance, OpenActionResult, async_trait};
use oaytmd_companion::models::request::CommandRequest;

use crate::actions::send_command;

pub struct PlayPauseAction;

#[async_trait]
impl Action for PlayPauseAction {
	const UUID: &'static str = "justmangoou.oaytmd.playPause";
	type Settings = ();

	async fn key_up(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		send_command(instance, &CommandRequest::PlayPause).await
	}
}

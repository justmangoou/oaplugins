use std::collections::HashMap;

use openaction::{Action, Instance, OpenActionResult, async_trait};
use ytmd_companion::models::request::CommandRequest;

use crate::client::send_command;

pub struct ShuffleAction;

#[async_trait]
impl Action for ShuffleAction {
	const UUID: &'static str = "justmangoou.oaytmd.shuffle";
	type Settings = HashMap<String, String>;

	async fn key_up(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		send_command(instance, &CommandRequest::Shuffle(true)).await
	}
}

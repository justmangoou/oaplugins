use std::collections::HashMap;

use openaction::{Action, Instance, OpenActionResult, async_trait};
use oaytmd_companion::models::request::CommandRequest;

use crate::actions::send_command;

pub struct PreviousAction;

#[async_trait]
impl Action for PreviousAction {
	const UUID: &'static str = "justmangoou.oaytmd.previous";
	type Settings = HashMap<String, String>;

	async fn key_up(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		send_command(instance, &CommandRequest::Previous).await
	}
}

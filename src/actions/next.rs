use std::collections::HashMap;

use openaction::{Action, Instance, OpenActionResult, async_trait};
use oaytmd_companion::models::request::CommandRequest;

use crate::actions::send_command;

pub struct NextAction;

#[async_trait]
impl Action for NextAction {
	const UUID: &'static str = "justmangoou.oaytmd.next";
	type Settings = HashMap<String, String>;

	async fn key_up(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		send_command(instance, &CommandRequest::Next).await
	}
}

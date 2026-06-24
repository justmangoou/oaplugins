use std::collections::HashMap;

use openaction::{Action, Instance, OpenActionResult, async_trait};
use ytmd_companion::models::{RepeatMode, request::CommandRequest};

use crate::client::{YTMD_PLAYER, send_command};

async fn set_new_state(instance: &Instance, mode: &RepeatMode) -> OpenActionResult<()> {
	let new_state = match mode {
		RepeatMode::Unknown | RepeatMode::None => 0,
		RepeatMode::All => 1,
		RepeatMode::One => 2,
	};

	instance.set_state(new_state).await
}

pub struct RepeatAction;

#[async_trait]
impl Action for RepeatAction {
	const UUID: &'static str = "justmangoou.oaytmd.repeat";
	type Settings = HashMap<String, String>;

	async fn did_receive_settings(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		let player = YTMD_PLAYER.read().await;
		set_new_state(instance, &player.repeat_mode).await
	}

	async fn will_appear(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		let player = YTMD_PLAYER.read().await;
		set_new_state(instance, &player.repeat_mode).await
	}

	async fn key_up(
		&self,
		instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		let player = YTMD_PLAYER.read().await;
		let new_mode = match player.repeat_mode {
			RepeatMode::Unknown => RepeatMode::None,
			RepeatMode::None => RepeatMode::All,
			RepeatMode::All => RepeatMode::One,
			RepeatMode::One => RepeatMode::None,
		};

		match send_command(instance, &CommandRequest::RepeatMode(new_mode.clone())).await {
			Ok(_) => {
				set_new_state(instance, &new_mode).await?;
			}
			Err(error) => {
				log::error!("Failed to send repeat mode command: {}", error);
				instance.show_alert().await?;
			}
		}

		Ok(())
	}
}

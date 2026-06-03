use openaction::{Instance, OpenActionResult};
use oaytmd_companion::models::request::CommandRequest;

use crate::client::ytmd_client;

mod next;
mod play_pause;
mod previous;
mod repeat;
mod shuffle;
mod volume_control;

pub use next::NextAction;
pub use play_pause::PlayPauseAction;
pub use previous::PreviousAction;
pub use repeat::RepeatAction;
pub use shuffle::ShuffleAction;
pub use volume_control::{VolumeControlAction, VOLUME_CHANGE_ACCUMULATOR};

async fn send_command(instance: &Instance, command: &CommandRequest) -> OpenActionResult<()> {
	let client_lock = ytmd_client().load();

	let client = match client_lock.as_ref() {
		Some(c) => c,
		None => {
			log::error!("YouTube Music client is not connected");
			instance.show_alert().await?;
			return Ok(());
		}
	};

	match client.send_command(command).await {
		Ok(_) => {
			// NO-OP
		}
		Err(error) => {
			log::error!("Failed to send command {:?}: {}", command, error);
			instance.show_alert().await?;
		}
	}

	Ok(())
}

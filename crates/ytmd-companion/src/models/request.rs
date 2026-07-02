use serde::{Deserialize, Serialize};

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/auth-requestcode.html#request>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthCodeRequest {
	pub app_id: String,
	pub app_name: String,
	pub app_version: String,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/auth-request.html#request>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
	pub app_id: String,
	pub code: String,
}

/// Minimized of <https://ytmdesktop.github.io/developer/companion-server/reference/v1/command.html#commands>
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "command", content = "data")]
pub enum CommandRequest {
	PlayPause,
	// Play,
	// Pause,
	// VolumeUp,
	// VolumeDown,
	SetVolume(u8),
	Mute,
	Unmute,
	// SeekTo(u64),
	Next,
	Previous,
	// RepeatMode(RepeatMode),
	Shuffle(bool),
	// PlayQueueIndex(u32),
	// ToggleLike,
	// ToggleDislike,
}

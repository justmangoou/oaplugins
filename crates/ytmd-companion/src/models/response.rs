use serde::Deserialize;

use super::{RepeatMode, Thumbnail, TrackState};

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/auth-requestcode.html#response>
#[derive(Deserialize, Debug)]
pub struct AuthCodeResponse {
	pub code: String,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/auth-request.html#response>
#[derive(Deserialize, Debug)]
pub struct AuthResponse {
	pub token: String,
}

/// Minimized of <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#response>
#[derive(Deserialize, Debug, Clone)]
pub struct StateResponse {
	pub player: Player,
	#[serde(rename = "playlistId", skip_serializing_if = "Option::is_none")]
	pub playlist_id: Option<String>,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#player-object>
#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Player {
	pub track_state: TrackState,
	pub video_progress: f32,
	pub volume: u32,
	pub muted: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub queue: Option<Queue>,
}

/// Minimized of <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#queue-object>
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
	pub autoplay: bool,
	pub items: Vec<QueueItem>,
	pub automix_items: Vec<QueueItem>,
	pub is_generating: bool,
	pub is_infinite: bool,
	pub repeat_mode: RepeatMode,
	// pub selected_item_index: Option<u32>,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#queue-item-object>
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueueItem {
	pub thumbnails: Vec<Thumbnail>,
	pub title: String,
	pub author: String,
	pub duration: Option<String>,
	pub selected: bool,
	pub video_id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub counterparts: Option<Vec<QueueItem>>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum WebsocketEvent {
	StateUpdate(StateResponse),
	Error(String),
}

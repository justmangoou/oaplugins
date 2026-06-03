use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Default, Debug, Clone)]
#[repr(i8)]
pub enum RepeatMode {
	Unknown = -1,
	#[default]
	None = 0,
	All = 1,
	One = 2,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#track-state-enum>
#[derive(Serialize_repr, Deserialize_repr, Default, Debug, Clone)]
#[repr(i8)]
pub enum TrackState {
	Unknown = -1,
	#[default]
	Paused = 0,
	Playing = 1,
	Buffering = 2,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#like-status-enum>
#[derive(Serialize_repr, Deserialize_repr, Default, Debug, Clone)]
#[repr(i8)]
pub enum LikeStatus {
	Unknown = -1,
	Displike = 0,
	#[default]
	Indifferent = 1,
	Like = 2,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#video-type-enum>
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(i8)]
pub enum VideoType {
	Unknown = -1,
	Audio = 0,
	Video = 1,
	Uploaded = 2,
	Podcast = 3,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#thumbnail-object>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thumbnail {
	pub url: String,
	pub width: u32,
	pub height: u32,
}

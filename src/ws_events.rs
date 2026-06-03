use std::sync::Arc;

use oaytmd_companion::models::response::{StateResponse, WebsocketEvent};

use crate::client::{PlayerWrapper, ytmd_player};

pub async fn handle_ws_event(item: WebsocketEvent) {
	match item {
		WebsocketEvent::StateUpdate(state) => apply_state_update(state).await,
		WebsocketEvent::Error(error) => {
			log::error!("Received error event: {}", error);
		}
	}
}

async fn apply_state_update(state: StateResponse) {
	ytmd_player().store(Arc::new(PlayerWrapper {
		muted: state.player.muted,
		volume: state.player.volume,
		repeat_mode: state
			.player
			.queue
			.map(|q| q.repeat_mode)
			.unwrap_or_default(),
	}));
}

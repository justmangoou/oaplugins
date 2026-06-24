use futures_util::FutureExt;
use rust_socketio::Payload;
use serde_json::Value;
use tokio::sync::broadcast;

use crate::{
	Error, Result,
	models::response::{StateResponse, WebsocketEvent},
};

pub struct SocketClient {
	#[allow(dead_code)]
	pub inner: rust_socketio::asynchronous::Client,
	pub events: broadcast::Sender<WebsocketEvent>,
}

impl SocketClient {
	pub async fn connect(base_url: impl Into<String>, token: Option<String>) -> Result<Self> {
		let (events, _) = broadcast::channel(128);

		let state_events = events.clone();
		let error_events = events.clone();

		let client = rust_socketio::asynchronous::ClientBuilder::new(base_url)
			.namespace("/api/v1/realtime")
			.transport_type(rust_socketio::TransportType::Websocket)
			.auth(serde_json::json!({
				"token": token
			}))
			.on("state-update", move |payload, _| {
				let events = state_events.clone();

				async move {
					match payload {
						Payload::Text(values) => {
							let value = values.iter().find(|v| v.is_object());

							if let Some(value) = value {
								match serde_json::from_value::<StateResponse>(value.clone()) {
									Ok(state) => {
										let _ = events.send(WebsocketEvent::StateUpdate(state));
									}
									Err(err) => {
										let _ = events.send(WebsocketEvent::Error(err.to_string()));
									}
								}
							}
						}
						other => {
							let _ = events.send(WebsocketEvent::Error(format!(
								"Unexpected payload for state-update: {other:?}"
							)));
						}
					}
				}
				.boxed()
			})
			.on("error", move |payload, _| {
				let events = error_events.clone();

				async move {
					let message = match payload {
						Payload::Text(values) => values
							.first()
							.map(Value::to_string)
							.unwrap_or_else(|| "Unknown error".to_string()),
						other => format!("{other:?}"),
					};

					let _ = events.send(WebsocketEvent::Error(message));
				}
				.boxed()
			})
			.connect()
			.await
			.map_err(|_| Error::UnableToConnect)?;

		Ok(Self {
			inner: client,
			events,
		})
	}

	pub fn subscribe(&self) -> broadcast::Receiver<WebsocketEvent> {
		self.events.subscribe()
	}
}

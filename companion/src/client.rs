use arc_swap::{ArcSwap, ArcSwapOption};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
	Error, Result,
	models::{
		request::{AuthCodeRequest, AuthRequest, CommandRequest},
		response::{AuthCodeResponse, AuthResponse, StateResponse, WebsocketEvent},
	},
};

mod rest;
mod socket;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ClientSettings {
	pub app_id: String,
	pub app_name: String,
	pub app_version: String,
	pub host: String,
	pub port: u16,
	pub token: Option<String>,
}

impl ClientSettings {
	pub fn base_url(&self) -> String {
		format!("http://{}:{}", self.host, self.port)
	}

	pub fn api_url(&self) -> String {
		format!("{}{}", self.base_url(), "/api/v1")
	}
}

pub struct Client {
	pub settings: ArcSwap<ClientSettings>,
	rest: ArcSwap<rest::RestClient>,
	socket: ArcSwapOption<socket::SocketClient>,
}

impl Client {
	pub fn new(settings: ClientSettings) -> Self {
		let rest_client = rest::RestClient::new(settings.api_url());

		Self {
			settings: ArcSwap::from_pointee(settings),
			rest: ArcSwap::from_pointee(rest_client),
			socket: ArcSwapOption::from(None),
		}
	}

	pub async fn connect(&self, force_reauth: bool) -> Result<()> {
	    let current_settings = self.settings.load();

		let settings = if force_reauth || current_settings.token.is_none() {
			let code_resp = self.auth_request_code().await?;
			let auth_resp = self.auth_request(code_resp.code).await?;

			let mut new_settings = (**current_settings).clone();
			new_settings.token = Some(auth_resp.token.clone());

			let new_rest = rest::RestClient::new(new_settings.api_url());

			let arc_settings = Arc::new(new_settings);
			self.settings.store(arc_settings.clone());
			self.rest.store(Arc::new(new_rest));

			arc_settings
		} else {
			arc_swap::Guard::into_inner(current_settings)
		};

		// Reads fresh host, port, and token cleanly from unified Arc context
		let socket_client =
			socket::SocketClient::connect(&settings.base_url(), settings.token.clone()).await;

		self.socket.store(Some(Arc::new(socket_client)));

		Ok(())
	}

	pub fn setup_event_handler<F, Fut>(&self, func: F) -> Result<tokio::task::JoinHandle<()>>
	where
		F: Fn(WebsocketEvent) -> Fut + Send + Sync + 'static,
		Fut: Future<Output = ()> + Send + 'static,
	{
		let socket = self
			.socket
			.load()
			.as_ref()
			.cloned()
			.ok_or(Error::SocketClientNotConnected)?;
		let mut rx = socket.subscribe();

		Ok(tokio::spawn(async move {
			while let Ok(event) = rx.recv().await {
				func(event).await;
			}
		}))
	}

	pub fn set_settings(&self, new_settings: ClientSettings) {
		let new_rest_client = rest::RestClient::new(new_settings.api_url());

		self.settings.store(Arc::new(new_settings));
		self.rest.store(Arc::new(new_rest_client));
	}

	pub async fn get_state(&self) -> Result<StateResponse> {
		let settings = self.settings.load();
		let rest = self.rest.load();

		rest.get("/state", settings.token.as_deref()).await
	}

	async fn auth_request_code(&self) -> Result<AuthCodeResponse> {
		let settings = self.settings.load();
		let rest = self.rest.load();

		rest.post::<AuthCodeRequest, AuthCodeResponse>(
			"/auth/requestcode",
			&AuthCodeRequest {
				app_id: settings.app_id.clone(),
				app_name: settings.app_name.clone(),
				app_version: settings.app_version.clone(),
			},
			settings.token.as_deref(),
		)
		.await?
		.ok_or_else(|| crate::Error::UnexpectedResponse("No auth code received".into()))
	}

	async fn auth_request(&self, code: String) -> Result<AuthResponse> {
		let settings = self.settings.load();
		let rest = self.rest.load();

		rest.post::<AuthRequest, AuthResponse>(
			"/auth/request",
			&AuthRequest {
				app_id: settings.app_id.clone(),
				code,
			},
			settings.token.as_deref(),
		)
		.await?
		.ok_or_else(|| crate::Error::UnexpectedResponse("No auth token received".into()))
	}

	pub async fn send_command(&self, command: &CommandRequest) -> Result<()> {
		let settings = self.settings.load();
		let rest = self.rest.load();

		rest.post::<_, serde_json::Value>("/command", command, settings.token.as_deref())
			.await?;

		Ok(())
	}
}

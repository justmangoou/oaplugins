use serde::{Deserialize, Serialize};

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
	pub settings: ClientSettings,
	rest: rest::RestClient,
	socket: Option<socket::SocketClient>,
}

impl Client {
	pub fn new(settings: ClientSettings) -> Self {
		let rest = rest::RestClient::new(settings.api_url());

		Self {
			settings,
			rest,
			socket: None,
		}
	}

	pub async fn connect(&mut self, force_reauth: bool) -> Result<()> {
		if force_reauth || self.settings.token.is_none() {
			let code_resp = self.auth_request_code().await?;
			let auth_resp = self.auth_request(code_resp.code).await?;

			self.settings.token = Some(auth_resp.token);
			self.rest = rest::RestClient::new(self.settings.api_url());
		}

		let socket =
			socket::SocketClient::connect(&self.settings.base_url(), self.settings.token.clone())
				.await?;

		self.socket = Some(socket);

		Ok(())
	}

	pub fn setup_event_handler<F, Fut>(&self, func: F) -> Result<tokio::task::JoinHandle<()>>
	where
		F: Fn(WebsocketEvent) -> Fut + Send + Sync + 'static,
		Fut: Future<Output = ()> + Send + 'static,
	{
		let socket = self
			.socket
			.as_ref()
			.ok_or(Error::SocketClientNotConnected)?;

		let mut rx = socket.subscribe();

		Ok(tokio::spawn(async move {
			while let Ok(event) = rx.recv().await {
				func(event).await;
			}
		}))
	}

	pub fn set_settings(&mut self, settings: ClientSettings) {
		self.rest = rest::RestClient::new(settings.api_url());
		self.settings = settings;
	}

	pub async fn get_state(&self) -> Result<StateResponse> {
		self.rest
			.get("/state", self.settings.token.as_deref())
			.await
	}

	async fn auth_request_code(&self) -> Result<AuthCodeResponse> {
		self.rest
			.post::<AuthCodeRequest, AuthCodeResponse>(
				"/auth/requestcode",
				&AuthCodeRequest {
					app_id: self.settings.app_id.clone(),
					app_name: self.settings.app_name.clone(),
					app_version: self.settings.app_version.clone(),
				},
				self.settings.token.as_deref(),
			)
			.await?
			.ok_or_else(|| Error::UnexpectedResponse("No auth code received".into()))
	}

	async fn auth_request(&self, code: String) -> Result<AuthResponse> {
		self.rest
			.post::<AuthRequest, AuthResponse>(
				"/auth/request",
				&AuthRequest {
					app_id: self.settings.app_id.clone(),
					code,
				},
				self.settings.token.as_deref(),
			)
			.await?
			.ok_or_else(|| Error::UnexpectedResponse("No auth token received".into()))
	}

	pub async fn send_command(&self, command: &CommandRequest) -> Result<()> {
		self.rest
			.post::<_, serde_json::Value>("/command", command, self.settings.token.as_deref())
			.await?;

		Ok(())
	}
}

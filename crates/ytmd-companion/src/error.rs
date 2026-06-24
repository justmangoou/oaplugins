#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("HTTP error {status_code}{}", .message.as_ref().map(|m| format!(": {m}")).unwrap_or_default())]
	Upstream {
		status_code: u16,
		message: Option<String>,
	},

	#[error("Unable to connect to YTMD")]
	UnableToConnect,

	#[error("Socket client not connected")]
	SocketClientNotConnected,

	#[error("Unexpected response: {0}")]
	UnexpectedResponse(String),

	#[error("{0}")]
	Reqwest(#[from] reqwest::Error),
}

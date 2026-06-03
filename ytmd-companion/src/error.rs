#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("{0}")]
	Reqwest(#[from] reqwest::Error),

	#[error("HTTP error {status_code}{}", .message.as_ref().map(|m| format!(": {m}")).unwrap_or_default())]
	Upstream {
		status_code: u16,
		message: Option<String>,
	},

	#[error("Socket client not connected")]
	SocketClientNotConnected,

	#[error("Unexpected response: {0}")]
	UnexpectedResponse(String),
}

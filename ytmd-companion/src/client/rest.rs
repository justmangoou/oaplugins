use serde::Serialize;

use crate::Result;

pub(super) struct RestClient {
	client: reqwest::Client,
	base_url: String,
}

impl RestClient {
	pub fn new(base_url: impl Into<String>) -> Self {
		let client = reqwest::ClientBuilder::new()
			.timeout(std::time::Duration::from_secs(5))
			.build()
			.expect("Failed to build HTTP client");

		Self {
			client,
			base_url: base_url.into(),
		}
	}

	/// GET request to the API
	///
	/// # Generic Arguments
	/// * `T` - The response type
	///
	/// # Errors
	/// Returns an error if the request fails or the response is not successful
	pub async fn get<T>(&self, path: &str, token: Option<&str>) -> Result<T>
	where
		T: serde::de::DeserializeOwned,
	{
		let url = format!("{}{}", self.base_url, path);
		let response = self
			.build_request(self.client.get(url), token)
			.send()
			.await?;

		self.handle_response(response).await
	}

	/// POST request to the API
	///
	/// # Generic Arguments
	/// * `R` - The response type
	/// * `B` - The request body type
	///
	/// # Errors
	/// Returns an error if the request fails or the response is not successful
	pub async fn post<B, R>(&self, path: &str, body: &B, token: Option<&str>) -> Result<Option<R>>
	where
		R: serde::de::DeserializeOwned,
		B: Serialize,
	{
		let url = format!("{}{}", self.base_url, path);
		let response = self
			.build_request(self.client.post(url), token)
			.json(body)
			.send()
			.await?;

		let status = response.status();

		// Handle 204 No Content - empty response
		if status == reqwest::StatusCode::NO_CONTENT {
			return Ok(None);
		}

		self.handle_response::<R>(response).await.map(Some)
	}

	/// Add authorization header if token is provided
	fn build_request(
		&self,
		req: reqwest::RequestBuilder,
		token: Option<&str>,
	) -> reqwest::RequestBuilder {
		match token {
			Some(t) => req.header(reqwest::header::AUTHORIZATION, t),
			None => req,
		}
	}

	/// Handle the HTTP response and deserialize or return error
	async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T>
	where
		T: serde::de::DeserializeOwned,
	{
		let status = response.status();

		if !status.is_success() {
			// Try to read the error body, but if it fails, create a generic error
			let body = response.text().await.ok();
			return Err(crate::Error::Upstream {
				status_code: status.as_u16(),
				message: body,
			});
		}

		response.json::<T>().await.map_err(crate::Error::Reqwest)
	}
}

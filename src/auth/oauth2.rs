use reqwest::Error;
use serde::Deserialize;
use crate::auth::client::ClientCredentials;

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

pub struct OAuth2Client {
    client: reqwest::Client,
    credentials: ClientCredentials,
    token_url: String,
}

impl OAuth2Client {
    pub fn new(credentials: ClientCredentials, token_url: String) -> Self {
        OAuth2Client {
            client: reqwest::Client::new(),
            credentials,
            token_url,
        }
    }

    pub async fn get_access_token(&self) -> Result<TokenResponse, Error> {
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.credentials.client_id),
            ("client_secret", &self.credentials.client_secret),
        ];

        let response = self
            .client
            .post(&self.token_url)
            .form(&params)
            .send()
            .await?;

        let token_response: TokenResponse = response.json().await?;
        Ok(token_response)
    }
}
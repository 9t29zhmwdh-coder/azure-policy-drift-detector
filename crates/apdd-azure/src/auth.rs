use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

const MANAGEMENT_BASE: &str = "https://management.azure.com";
const TOKEN_URL: &str = "https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token";

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Debug, Clone)]
pub struct AzureClient {
    pub tenant_id: String,
    pub subscription_id: String,
    client_id: String,
    client_secret: String,
    pub http: reqwest::Client,
    token: Arc<RwLock<Option<String>>>,
}

impl AzureClient {
    pub fn from_env() -> Result<Self> {
        let tenant_id = std::env::var("AZURE_TENANT_ID")
            .map_err(|_| anyhow!("AZURE_TENANT_ID not set"))?;
        let client_id = std::env::var("AZURE_CLIENT_ID")
            .map_err(|_| anyhow!("AZURE_CLIENT_ID not set"))?;
        let client_secret = std::env::var("AZURE_CLIENT_SECRET")
            .map_err(|_| anyhow!("AZURE_CLIENT_SECRET not set"))?;
        let subscription_id = std::env::var("AZURE_SUBSCRIPTION_ID")
            .map_err(|_| anyhow!("AZURE_SUBSCRIPTION_ID not set"))?;

        Ok(Self {
            tenant_id,
            subscription_id,
            client_id,
            client_secret,
            http: reqwest::Client::new(),
            token: Arc::new(RwLock::new(None)),
        })
    }

    async fn acquire_token(&self) -> Result<String> {
        let url = TOKEN_URL.replace("{tenant}", &self.tenant_id);
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("scope", "https://management.azure.com/.default"),
        ];
        let resp: TokenResponse = self
            .http
            .post(&url)
            .form(&params)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.access_token)
    }

    pub async fn token(&self) -> Result<String> {
        {
            let guard = self.token.read().await;
            if let Some(t) = guard.as_ref() {
                return Ok(t.clone());
            }
        }
        let t = self.acquire_token().await?;
        *self.token.write().await = Some(t.clone());
        Ok(t)
    }

    pub fn management_url(&self, path: &str) -> String {
        format!("{}{}", MANAGEMENT_BASE, path)
    }

    pub async fn post_json<B: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<R> {
        debug!("POST {}", url);
        let token = self.token().await?;
        let resp = self
            .http
            .post(url)
            .bearer_auth(&token)
            .json(body)
            .send()
            .await?
            .error_for_status()?
            .json::<R>()
            .await?;
        Ok(resp)
    }
}

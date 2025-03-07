use super::endpoints::EnvVarsEndpoint;
use super::models::EnvVar;
use crate::error::Result;
use reqwest::Client;

pub struct CodesphereClient {
    client: Client,
    api_key: String,
}

impl CodesphereClient {
    pub fn new(api_key: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_key: api_key.to_string(),
        }
    }

    pub async fn set_env_vars(&self, workspace_id: &str, env_vars: Vec<EnvVar>) -> Result<()> {
        EnvVarsEndpoint::set_vars(
            &self.client,
            &self.api_key,
            Some(workspace_id.to_string()),
            env_vars,
        )
        .await
    }
}

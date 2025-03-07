use super::common::Endpoint;
use crate::api::models::EnvVar;
use crate::error::{endpoints::EnvVarsError, Result};
use reqwest::Client;

pub struct EnvVarsEndpoint;

impl Endpoint for EnvVarsEndpoint {
    fn endpoint_url(workspace_id: &str) -> String {
        format!(
            "https://codesphere.com/api/workspaces/{}/env-vars",
            workspace_id
        )
    }
}

impl EnvVarsEndpoint {
    pub async fn set_vars(
        client: &Client,
        api_key: &str,
        workspace_id: Option<String>,
        env_vars: Vec<EnvVar>,
    ) -> Result<()> {
        let workspace_id = workspace_id.ok_or(EnvVarsError::MissingWorkspaceId)?;

        let url = Self::endpoint_url(&workspace_id); // Hier wird url definiert

        let response = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&env_vars)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            println!("Successfully set environment variables!");
            Ok(())
        } else {
            let error_text = response.text().await?;
            println!("Error setting environment variables: {}", error_text);
            Err(EnvVarsError::ApiError {
                status_code: status.as_u16(),
                message: error_text,
            }
            .into())
        }
    }
}

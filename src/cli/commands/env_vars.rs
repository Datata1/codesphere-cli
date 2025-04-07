use super::types::SetEnvVarsArgs;
use crate::api::CodesphereClient;
use crate::error::{endpoints::EnvVarsError, Result};
use crate::utils::readers::env_vars::EnvFileReader;
use dotenv::dotenv;

fn get_api_key(cli_api_key: Option<String>) -> Result<String> {
    if let Some(key) = cli_api_key {
        return Ok(key);
    }

    if let Ok(key) = std::env::var("CODESPHERE_API_KEY") {
        return Ok(key);
    }

    dotenv().ok();
    if let Ok(key) = std::env::var("CODESPHERE_API_KEY") {
        return Ok(key);
    }

    Err(EnvVarsError::MissingApiKey.into())
}

pub async fn handle_set_env_vars(args: SetEnvVarsArgs) -> Result<()> {
    let api_key = get_api_key(args.api_key)?;

    let workspace_id = args
        .workspace_id
        .or_else(|| std::env::var("WORKSPACE_ID").ok())
        .ok_or_else(|| EnvVarsError::MissingWorkspaceId)?;

    let env_vars = EnvFileReader::read(args.env_file)?;
    let client = CodesphereClient::new(&api_key);
    client.set_env_vars(&workspace_id, env_vars).await
}

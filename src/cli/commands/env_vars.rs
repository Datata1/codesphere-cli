use super::types::SetEnvVarsArgs;
use crate::api::CodesphereClient;
use crate::error::{endpoints::EnvVarsError, Result};
use crate::utils::readers::env_vars::EnvFileReader; // Geändert von config zu utils

pub async fn handle_set_env_vars(args: SetEnvVarsArgs) -> Result<()> {
    let workspace_id = args
        .workspace_id
        .or_else(|| std::env::var("WORKSPACE_ID").ok())
        .ok_or_else(|| EnvVarsError::MissingWorkspaceId)?;

    let env_vars = EnvFileReader::read(&args.env_file)?;
    let client = CodesphereClient::new(&args.api_key);
    client.set_env_vars(&workspace_id, env_vars).await
}

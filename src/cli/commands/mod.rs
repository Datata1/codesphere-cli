mod env_vars;
mod types;

use crate::error::Result;
pub use types::{Cli, Commands};

pub async fn execute_command(command: &Commands) -> Result<()> {
    match command {
        Commands::SetEnvVars(args) => env_vars::handle_set_env_vars(args.clone()).await,
    }
}

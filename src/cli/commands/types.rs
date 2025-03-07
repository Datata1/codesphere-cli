use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Set environment variables in Codesphere
    #[command(name = "set-env-vars")]
    SetEnvVars(SetEnvVarsArgs),
}

#[derive(clap::Args, Debug, Clone)]
pub struct SetEnvVarsArgs {
    #[arg(short = 'e', long = "env-file")]
    pub env_file: std::path::PathBuf,

    #[arg(short = 'k', long = "api-key")]
    pub api_key: String,

    #[arg(short = 'w', long = "workspace-id")]
    pub workspace_id: Option<String>,
}

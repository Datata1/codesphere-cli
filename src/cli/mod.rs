mod commands;

use crate::error::Result;
use clap::Parser;
pub use commands::Cli;

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    commands::execute_command(&cli.command).await
}

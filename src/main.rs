mod api;
mod cli;
mod error;
mod utils;

use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await
}

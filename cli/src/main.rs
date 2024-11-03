mod cli;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    Cli::parse().execute().await
}

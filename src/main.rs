mod args;
mod devtool;
mod fs;
mod keys;
mod tendermint;
mod tx;

use clap::Parser;
use color_eyre::eyre::Result;
use namada_apps::logging;
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    logging::init_from_env_or(LevelFilter::INFO)?;
    let cmd = args::Devtool::parse().command;
    devtool::run(cmd).await
}

use std::{net::SocketAddr, path::PathBuf};

use anyhow::{Context, anyhow};
use clap::Parser;
use lumen_auth::config::Config;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, action)]
    config_file: PathBuf,

    #[clap(long, action)]
    http_address: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let config_file = &args.config_file;
    let config_file_contents = std::fs::read_to_string(config_file)
        .with_context(|| format!("read config file {:?}", config_file))?;
    let mut config: Config = toml::from_str(&config_file_contents)
        .with_context(|| format!("parse config file {:?}", config_file))?;

    config.dropshot.bind_address = args.http_address;

    let log = config
        .log
        .to_logger("auth-server")
        .context("failed to create logger")?;

    let dropshot_server = lumen_auth::start_server(
        log,
        &config.dropshot,
        config.keys,
        config.database,
    )
    .await?;

    dropshot_server
        .await
        .map_err(|error_message| anyhow!("server exiting: {}", error_message))
}

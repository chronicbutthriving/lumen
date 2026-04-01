use std::{net::SocketAddr, path::PathBuf};

use anyhow::{Context, anyhow};
use clap::Parser;
use serde::Deserialize;
use slog::info;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, action)]
    config_file: PathBuf,

    #[clap(long, action)]
    http_address: SocketAddr,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: lumen_common::db::pool::DatabaseConfig,
    pub log: dropshot::ConfigLogging,
    pub dropshot: dropshot::ConfigDropshot,
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
    eprintln!("{:?}", config);

    let log = config
        .log
        .to_logger("auth-server")
        .context("failed to create logger")?;

    info!(&log, "config";
        "config" => ?config,
    );

    let dropshot_server =
        lumen_auth::start_server(log, &config.dropshot, config.database)
            .await?;

    dropshot_server
        .await
        .map_err(|error_message| anyhow!("server exiting: {}", error_message))
}

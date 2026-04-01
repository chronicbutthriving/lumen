use anyhow::anyhow;
use slog::o;

pub mod config;
mod context;
mod http_entrypoints;
mod auth;

pub async fn start_server(
    log: slog::Logger,
    dropshot_config: &dropshot::ConfigDropshot,
    keys_config: Vec<config::AsymmetricKey>,
    _database_config: lumen_common::db::pool::DatabaseConfig,
) -> Result<dropshot::HttpServer<context::Context>, anyhow::Error> {
    let http_api = http_entrypoints::api();
    let http_api_context = context::Context::new(
        None,
        keys_config,
    );

    let server = dropshot::ServerBuilder::new(
        http_api,
        http_api_context,
        log.new(o!("component" => "http")),
    )
    .config(dropshot_config.clone())
    .version_policy(dropshot::VersionPolicy::Dynamic(Box::new(
        dropshot::ClientSpecifiesVersionInHeader::new(
            lumen_common::api::VERSION_HEADER,
            lumen_auth_api::VERSION_INITIAL,
        ),
    )))
    .start()
    .map_err(|error| anyhow!("setting up HTTP server: {:#}", error))?;

    Ok(server)
}

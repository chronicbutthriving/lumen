use anyhow::anyhow;
use lumen_storage_db::dbs::MockStore;
use slog::o;

mod context;
mod http_entrypoints;

pub async fn start_server(
    log: slog::Logger,
    dropshot_config: &dropshot::ConfigDropshot,
) -> Result<dropshot::HttpServer<context::Context>, anyhow::Error> {
    let http_api = http_entrypoints::api();
    let http_api_context = context::Context::new(MockStore::new());

    let server = dropshot::ServerBuilder::new(
        http_api,
        http_api_context,
        log.new(o!("component" => "http"))
    )
        .config(dropshot_config.clone())
        .version_policy(dropshot::VersionPolicy::Dynamic(Box::new(
            dropshot::ClientSpecifiesVersionInHeader::new(
                lumen_common::api::VERSION_HEADER,
                lumen_storage_api::VERSION_INITIAL,
            ),
        )))
        .start()
        .map_err(|error| anyhow!("setting up HTTP server: {:#}", error))?;

    Ok(server)
}

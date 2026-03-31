use anyhow::anyhow;
use slog::o;

mod server;

pub async fn start_server(
    log: slog::Logger,
    dropshot_config: &dropshot::ConfigDropshot,
) -> Result<dropshot::HttpServer<server::Context>, anyhow::Error> {
    let http_api = server::api();
    let http_api_context = server::Context::new();

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

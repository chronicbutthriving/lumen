use dropshot::{EndpointTagPolicy, HttpError, HttpResponseOk, RequestContext};
use dropshot_api_manager_types::api_versions;
use lumen_storage_types_versions::latest;

api_versions!([
    (1, INITIAL),
]);

#[dropshot::api_description {
    tag_config = {
        allow_other_tags = false,
        policy = EndpointTagPolicy::ExactlyOne,
        tags = {
            "system/status" = {
                description = "Endpoints related to system health",
            },
        }
    }
}]
pub trait StorageApi {
    type Context;

    /// Check API reachability and basic health.
    #[endpoint(
        method = GET,
        path = "/v1/ping",
        tags = ["system/status"]
    )]
    async fn ping(
        _rqctx: RequestContext<Self::Context>,
    ) -> Result<HttpResponseOk<latest::system::Ping>, HttpError>;
}

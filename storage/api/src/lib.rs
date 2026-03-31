use dropshot::{EndpointTagPolicy, HttpError, HttpResponseOk, RequestContext};
use dropshot_api_manager_types::api_versions;
use lumen_storage_types_versions::latest;

api_versions!([(1, INITIAL),]);

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

    /// List known objects.
    #[endpoint(
        method = GET,
        path = "/objects",
        tags = ["objects"]
    )]
    async fn list_objects(
        rqctx: RequestContext<Self::Context>,
    ) -> Result<HttpResponseOk<Vec<latest::object::StorageObject>>, HttpError>;

    /// Get a specific object by ID.
    #[endpoint(
        method = GET,
        path = "/objects/{id}",
        tags = ["objects"],
    )]
    async fn get_object(
        rqctx: RequestContext<Self::Context>,
        path_params: dropshot::Path<latest::object::GetObjectParams>,
    ) -> Result<HttpResponseOk<latest::object::StorageObject>, HttpError>;

    /// Check API reachability and basic health.
    #[endpoint(
        method = GET,
        path = "/ping",
        tags = ["system/status"]
    )]
    async fn ping(
        rqctx: RequestContext<Self::Context>,
    ) -> Result<HttpResponseOk<latest::system::Ping>, HttpError>;
}

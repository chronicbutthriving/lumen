use dropshot::{EndpointTagPolicy, HttpError, HttpResponseOk, RequestContext};
use dropshot_api_manager_types::api_versions;
use lumen_auth_types_versions::latest;

api_versions!([(1, INITIAL),]);

#[dropshot::api_description {
    tag_config = {
        allow_other_tags = false,
        policy = EndpointTagPolicy::ExactlyOne,
        tags = {
            "users" = {
                description = "Endpoints related to users & user management.",
            },
            "m2m" = {
                description = "Endpoints related to machine-to-machine (M2M) authentication.",
            },
            "system" = {
                description = "Endpoints related to the system in general.",
            },
            "system/status" = {
                description = "Endpoints related to system health.",
            },
        }
    }
}]
pub trait AuthApi {
    type Context;

    /// List the JWKS for the system.
    #[endpoint(
        method = GET,
        path = "/.well-known/jwks.json",
        tags = ["system"],
    )]
    async fn get_jwks(
        rqctx: RequestContext<Self::Context>,
    ) -> Result<HttpResponseOk<latest::system::JwksResponse>, HttpError>;

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

use dropshot::{
    EndpointTagPolicy, HttpError, HttpResponseOk, Path, RequestContext, TypedBody
};
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

    /// List the users in the system.
    #[endpoint(
        method = GET,
        path = "/v1/users",
        tags = ["users"],
    )]
    async fn list_users(
        rqctx: RequestContext<Self::Context>,
    ) -> Result<HttpResponseOk<Vec<latest::user::User>>, HttpError>;

    /// Invite a new user to the system by email.
    #[endpoint(
        method = POST,
        path = "/v1/users",
        tags = ["users"],
    )]
    async fn invite_user(
        rqctx: RequestContext<Self::Context>,
        body: TypedBody<latest::user::InviteUserRequest>,
    ) -> Result<HttpResponseOk<latest::user::InviteUserResponse>, HttpError>;

    /// Get a user in the system by ID.
    #[endpoint(
        method = GET,
        path = "/v1/users/{user_id}",
        tags = ["users"],
    )]
    async fn get_user(
        rqctx: RequestContext<Self::Context>,
        path: Path<latest::user::UserPathParams>,
    ) -> Result<HttpResponseOk<Vec<latest::user::User>>, HttpError>;

    /// Update a user's password.
    #[endpoint(
        method = POST,
        path = "/v1/users/{user_id}/password",
        tags = ["users"],
    )]
    async fn update_user_password(
        rqctx: RequestContext<Self::Context>,
        path: Path<latest::user::UserPathParams>,
        body: TypedBody<latest::user::UpdateUserPasswordRequest>,
    ) -> Result<HttpResponseOk<()>, HttpError>;

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

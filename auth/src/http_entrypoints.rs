use dropshot::{ApiDescription, HttpError, HttpResponseOk, RequestContext};
use lumen_auth_api::AuthApi;
use lumen_auth_types_versions::latest;

use crate::context::Context;

pub type AuthApiDescription = ApiDescription<Context>;

pub fn api() -> AuthApiDescription {
    lumen_auth_api::auth_api_mod::api_description::<AuthApiImpl>()
        .expect("registered storage server endpoints")
}

pub enum AuthApiImpl {}

impl AuthApi for AuthApiImpl {
    type Context = super::context::Context;

    async fn get_jwks(
        _rqctx: RequestContext<Self::Context>,
    ) -> Result<HttpResponseOk<latest::system::JwksResponse>, HttpError> {
        Ok(HttpResponseOk(latest::system::JwksResponse { keys: Vec::new() }))
    }

    async fn ping(
        _rqctx: dropshot::RequestContext<Self::Context>,
    ) -> Result<
        dropshot::HttpResponseOk<latest::system::Ping>,
        dropshot::HttpError,
    > {
        Ok(dropshot::HttpResponseOk(
            latest::system::Ping {
                status: latest::system::PingStatus::Ok,
            },
        ))
    }
}

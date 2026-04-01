use dropshot::{
    ApiDescription, HttpError, HttpResponseOk, Path, RequestContext, TypedBody,
};
use jsonwebtoken::jwk::{AlgorithmParameters, PublicKeyUse};
use lumen_auth_api::AuthApi;
use lumen_auth_types_versions::{
    latest,
    v1::system::{Jwk, JwksResponse},
};

use crate::context::Context;

pub type AuthApiDescription = ApiDescription<Context>;

pub fn api() -> AuthApiDescription {
    lumen_auth_api::auth_api_mod::api_description::<AuthApiImpl>()
        .expect("registered storage server endpoints")
}

pub enum AuthApiImpl {}

impl AuthApi for AuthApiImpl {
    type Context = super::context::Context;

    async fn list_users(
        _rqctx: RequestContext<Self::Context>,
    ) -> Result<HttpResponseOk<Vec<latest::user::User>>, HttpError> {
        unimplemented!()
    }

    async fn get_user(
        _rqctx: RequestContext<Self::Context>,
        _path: Path<latest::user::UserPathParams>,
    ) -> Result<HttpResponseOk<Vec<latest::user::User>>, HttpError> {
        unimplemented!()
    }

    async fn invite_user(
        _rqctx: RequestContext<Self::Context>,
        _body: TypedBody<latest::user::InviteUserRequest>,
    ) -> Result<HttpResponseOk<latest::user::InviteUserResponse>, HttpError>
    {
        unimplemented!()
    }

    async fn update_user_password(
        _rqctx: RequestContext<Self::Context>,
        _path: Path<latest::user::UserPathParams>,
        _body: TypedBody<latest::user::UpdateUserPasswordRequest>,
    ) -> Result<HttpResponseOk<()>, HttpError> {
        unimplemented!()
    }

    async fn get_jwks(
        rqctx: RequestContext<Self::Context>,
    ) -> Result<HttpResponseOk<latest::system::JwksResponse>, HttpError> {
        let jwks = rqctx.context().jwks().await;

        let resp = JwksResponse {
            keys: jwks
                .keys
                .iter()
                .map(|jwk| {
                    let (algo, x) = match &jwk.algorithm {
                        AlgorithmParameters::OctetKeyPair(params) => {
                            ("OKP".to_string(), params.x.clone())
                        }
                        _ => panic!("Unexpected key type"),
                    };

                    Jwk {
                        kty: algo,
                        kid: jwk.common.key_id.as_ref().unwrap().clone(),
                        use_: match jwk.common.public_key_use {
                            Some(PublicKeyUse::Signature) => "sig".to_string(),
                            _ => panic!("Unexpected key use"),
                        },
                        x,
                    }
                })
                .collect(),
        };

        Ok(HttpResponseOk(resp))
    }

    async fn ping(
        _rqctx: dropshot::RequestContext<Self::Context>,
    ) -> Result<
        dropshot::HttpResponseOk<latest::system::Ping>,
        dropshot::HttpError,
    > {
        Ok(dropshot::HttpResponseOk(latest::system::Ping {
            status: latest::system::PingStatus::Ok,
        }))
    }
}

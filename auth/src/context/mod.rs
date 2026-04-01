use std::path::PathBuf;

use jsonwebtoken::jwk::JwkSet;

use crate::{config::AsymmetricKey, context::auth::AuthContext};

pub mod auth;

pub struct Context {
    auth: AuthContext,
}

impl Context {
    pub fn new(param_path: Option<PathBuf>, keys: Vec<AsymmetricKey>) -> Self {
        let jwks = JwkSet {
            keys: keys
                .iter()
                .filter_map(|key| key.resolve_jwk(param_path.clone()).ok())
                .collect::<Vec<_>>(),
        };

        Self { auth: AuthContext::new(jwks) }
    }

    pub async fn jwks(&self) -> &JwkSet {
        self.auth.jwks().await
    }
}

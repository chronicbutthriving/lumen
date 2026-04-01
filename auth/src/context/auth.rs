use jsonwebtoken::jwk::JwkSet;

pub struct AuthContext {
    jwt: JwtContext,
}

impl AuthContext {
    pub fn new(jwks: JwkSet) -> Self {
        Self {
            jwt: JwtContext { jwks },
        }
    }

    pub async fn jwks(&self) -> &JwkSet {
        &self.jwt.jwks
    }
}

pub struct JwtContext {
    pub jwks: JwkSet,
}

use digest::KeyInit;
use jwt::{SignWithKey};
use uuid::Uuid;
use crate::{api::AppContext};
use hmac::{Hmac};
use sha2::Sha256;
use time::OffsetDateTime;

const SESSION_LENGTH: time::Duration = time::Duration::weeks(1);

type HmacSha256 = Hmac<Sha256>;

pub struct AuthUser {
    pub user_id: Uuid
}

pub struct MaybeAuthUser(pub Option<AuthUser>);

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthUserClaims {
    user_id: Uuid,
    exp: i64
}

impl AuthUser {
    pub(in crate::api) fn to_jwt(&self, ctx: &AppContext) -> String {

        let hmac = Hmac::<Sha256>::new_from_slice(ctx.config.hmac_key.as_bytes())
            .expect("HMAC-SHA-256 can accept any key length");

        AuthUserClaims {
            user_id: self.user_id,
            exp: (OffsetDateTime::now_utc() + SESSION_LENGTH).unix_timestamp()
        }
            .sign_with_key(&hmac)
            .expect("HMAC signing should work")
    }
}
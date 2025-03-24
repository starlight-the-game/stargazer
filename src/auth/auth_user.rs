use crate::prisma::player::Data as PlayerData;
use axum_login::AuthUser;

impl AuthUser for PlayerData {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.numeric_id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.hashed_password.as_bytes()
    }
}

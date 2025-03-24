use crate::prisma;
use crate::prisma::player::Data as PlayerData;
use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use password_auth::generate_hash;
use prisma_client_rust::QueryError;
use std::sync::Arc;

use crate::dto::login::Login;
use prisma::*;

#[derive(Clone)]
pub struct Backend {
    pub prisma_client: Arc<PrismaClient>,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = PlayerData;
    type Credentials = Login;
    type Error = QueryError;

    async fn authenticate(
        &self,
        Login { email, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let db = self.prisma_client.as_ref();

        Ok(db
            .player()
            .find_first(vec![
                player::email::equals(email),
                player::hashed_password::equals(generate_hash(password)),
            ])
            .exec()
            .await?
        )
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let db = self.prisma_client.as_ref();

        Ok(db
            .player()
            .find_first(vec![player::numeric_id::equals(*user_id)])
            .exec()
            .await?)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;
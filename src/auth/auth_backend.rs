use crate::dto::login::Login;
use crate::prisma;
use crate::prisma::player::Data as PlayerData;
use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use password_auth::verify_password;
use prisma::*;
use prisma_client_rust::QueryError;
use std::sync::Arc;

#[derive(Clone)]
pub struct PrismaBackend {
    pub prisma_client: Arc<PrismaClient>,
}

impl PrismaBackend {
    pub fn new(prisma_client: Arc<PrismaClient>) -> Self {
        Self { prisma_client }
    }
}

#[async_trait]
impl AuthnBackend for PrismaBackend {
    type User = PlayerData;
    type Credentials = Login;
    type Error = QueryError;

    async fn authenticate(
        &self,
        Login { email, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let db = self.prisma_client.as_ref();

        let player = db
            .player()
            .find_first(vec![player::email::equals(email)])
            .exec()
            .await?
            .unwrap();

        let test_player = player.clone();

        if verify_password(password, test_player.hashed_password.as_str()).is_err() {
            return Ok(None);
        }

        Ok(Option::from(player))
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

pub type AuthSessionSimple = axum_login::AuthSession<PrismaBackend>;

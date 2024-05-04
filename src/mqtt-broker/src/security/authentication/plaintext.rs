use super::Authentication;
use crate::metadata::user::User;
use axum::async_trait;
use common_base::errors::RobustMQError;
use dashmap::DashMap;
use protocol::mqtt::Login;

pub struct Plaintext<'a> {
    login: Login,
    user_info: &'a DashMap<String, User>,
}

impl<'a> Plaintext<'a> {
    pub fn new(login: Login, user_info: &'a DashMap<String, User>) -> Self {
        return Plaintext { login, user_info };
    }
}

#[async_trait]
impl<'a> Authentication for Plaintext<'a> {
    async fn apply(&self) -> Result<bool, RobustMQError> {
        if let Some(user) = self.user_info.get(&self.login.username) {
            return Ok(user.password == self.login.password);
        }
        return Ok(false);
    }
}
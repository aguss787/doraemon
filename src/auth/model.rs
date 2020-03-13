use crate::auth::error::AuthError;
use serde::{Deserialize, Serialize};

pub type AuthResult<T> = Result<T, AuthError>;

pub type Token = String;
pub type RefreshToken = String;
pub type AuthCode = String;

#[derive(Deserialize, Serialize, Clone)]
pub struct TokenPayload {
    pub username: String,
    pub expiry_timestamp: u128,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthCodePayload {
    pub username: String,
    pub client_id: String,
    pub expiry_timestamp: u128,
}

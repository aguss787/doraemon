use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64;
use bcrypt;
use diesel::result::Error as DieselError;
use magic_crypt;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use serde::{Deserialize, Serialize};
use serde_json;

use error::AuthError;

use crate::database::handler::user::{NewUser, UserHandler};

mod error;

pub type AuthResult<T> = Result<T, AuthError>;

pub type Token = String;

#[derive(Deserialize, Serialize, Clone)]
pub struct TokenPayload {
    pub username: String,
    pub expiry_timestamp: u128,
}

pub struct Auth<'a> {
    cypher_key: &'a String,
    token_lifetime: u64,
    user_handler: UserHandler<'a>,
}

impl<'a> Auth<'a> {
    pub fn new(
        cypher_key: &'a String,
        token_lifetime: u64,
        user_handler: UserHandler<'a>,
    ) -> Auth<'a> {
        Auth {
            cypher_key,
            token_lifetime,
            user_handler,
        }
    }
}

impl<'a> Auth<'a> {
    pub fn authorize(&self, username: &String, password: &String) -> AuthResult<Token> {
        let potential_user = self.user_handler.get_by_username(username);

        match potential_user {
            Err(e) if e == DieselError::NotFound => Err(AuthError::NotFound),
            Ok(user) => {
                if verify(&user.password, password, &user.salt)? {
                    self.generate_token(&user.username)
                } else {
                    Err(AuthError::WrongPassword)
                }
            }
            Err(e) => Err(AuthError::DBError(e)),
        }
    }

    pub fn register(&self, username: &String, password: &String) -> AuthResult<()> {
        let salt = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

        let user = NewUser {
            username,
            password: &generate_password(password, &salt)?,
            salt: &salt,
        };

        match self.user_handler.new_user(&user) {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.to_string()
                    .contains("duplicate key value violates unique constraint")
                {
                    Err(AuthError::UserAlreadyExist)
                } else {
                    Err(AuthError::from(e))
                }
            }
        }
    }

    pub fn inspect(&self, encrypted_token: &String) -> AuthResult<TokenPayload> {
        let encrypted_token_bytes = base64::decode_config(encrypted_token, base64::URL_SAFE)?;
        let mut crypter = new_magic_crypt!(self.cypher_key.clone(), 256);
        let token_bytes = crypter.decrypt_bytes_to_bytes(&encrypted_token_bytes)?;

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

        let token: TokenPayload = serde_json::from_slice(&token_bytes)?;

        if token.expiry_timestamp < current_time {
            Err(AuthError::ExpiredToken)
        } else {
            Ok(token)
        }
    }

    fn generate_token(&self, username: &String) -> AuthResult<Token> {
        let expiry_time = SystemTime::now()
            .add(Duration::new(self.token_lifetime, 0))
            .duration_since(UNIX_EPOCH)?
            .as_millis();

        let token = TokenPayload {
            username: username.to_owned(),
            expiry_timestamp: expiry_time,
        };

        let token_bytes = serde_json::to_vec(&token)?;

        let mut crypter = new_magic_crypt!(self.cypher_key.clone(), 256);
        let encrypted_token_bytes = crypter.encrypt_bytes_to_bytes(&token_bytes);
        let encrypted_token = base64::encode_config(&encrypted_token_bytes, base64::URL_SAFE);
        Ok(encrypted_token)
    }
}

fn verify(key: &String, password: &String, salt: &String) -> AuthResult<bool> {
    Ok(bcrypt::verify(password.to_owned() + salt, key)?)
}

fn generate_password(password: &String, salt: &String) -> AuthResult<String> {
    Ok(bcrypt::hash(password.to_owned() + salt, 12)?)
}

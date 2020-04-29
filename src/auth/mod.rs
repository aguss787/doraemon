use std::ops::Add;
use std::panic;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64;
use bcrypt;
use diesel::result::Error as DieselError;
use magic_crypt;
use magic_crypt::MagicCrypt;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use serde_json;
use url::Url;

pub use error::AuthError;

use crate::auth::error::AuthError::{InvalidClientID, InvalidRedirectUri, InvalidToken};
use crate::auth::model::{
    AuthCode, AuthCodePayload, AuthResult, RefreshToken, Token, TokenPayload,
};
use crate::database::handler::client_credential::ClientCredentialHandler;
use crate::database::handler::user::{NewUser, User, UserHandler};

mod error;
pub mod model;

pub struct Auth<'a> {
    cypher_key: &'a String,
    token_lifetime: u64,
    auth_code_lifetime: u64,
    user_handler: UserHandler<'a>,
    client_credential_handler: ClientCredentialHandler<'a>,
}

impl<'a> Auth<'a> {
    pub fn new(
        cypher_key: &'a String,
        token_lifetime: u64,
        auth_code_lifetime: u64,
        user_handler: UserHandler<'a>,
        client_credential_handler: ClientCredentialHandler<'a>,
    ) -> Auth<'a> {
        Auth {
            cypher_key,
            token_lifetime,
            auth_code_lifetime,
            user_handler,
            client_credential_handler,
        }
    }
}

impl<'a> Auth<'a> {
    pub fn get_token(
        &self,
        username: &String,
        password: &String,
    ) -> AuthResult<(Token, RefreshToken)> {
        let potential_user = self.get_potential_user(username, password)?;
        let token = self.generate_token(&potential_user.username)?;
        let refresh_token = self.generate_refresh_token(&potential_user.username)?;

        Ok((token, refresh_token))
    }

    pub fn get_authorization_code(
        &self,
        username: &String,
        password: &String,
        client_id: &String,
        redirect_uri: &String,
    ) -> AuthResult<AuthCode> {
        if !self.check_redirect_uri(client_id, redirect_uri)? {
            return Err(InvalidRedirectUri);
        }
        let potential_user = self.get_potential_user(username, password)?;
        self.generate_auth_code(&potential_user.username, client_id)
    }

    pub fn exchange_token(
        &self,
        auth_code_string: &String,
        client_secret: &String,
    ) -> AuthResult<(Token, RefreshToken)> {
        let auth_code_bytes = self.decrypt(auth_code_string)?;

        let auth_code: AuthCodePayload = serde_json::from_slice(&auth_code_bytes)?;

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

        if auth_code.expiry_timestamp < current_time {
            return Err(AuthError::ExpiredToken)
        };

        let client_credential = match self.client_credential_handler.get_by_id(&auth_code.client_id) {
            Err(diesel::NotFound) => return Err(InvalidToken),
            o => o,
        }?;

        if !client_credential.secret.eq(client_secret) {
            return Err(InvalidClientID)
        };

        let token = self.generate_token(&auth_code.username)?;
        let refresh_token = self.generate_refresh_token(&auth_code.username)?;

        Ok((token, refresh_token))
    }

    pub fn check_redirect_uri(
        &self,
        client_id: &String,
        redirect_uri: &String,
    ) -> AuthResult<bool> {
        let client_credential = match self.client_credential_handler.get_by_id(client_id) {
            Err(diesel::NotFound) => return Err(InvalidClientID),
            o => o,
        }?;

        let mut parsed_url = Url::parse(redirect_uri).map_err(|_| InvalidRedirectUri)?;

        parsed_url.set_query(None);
        let url = parsed_url.into_string();

        return Ok(client_credential.redirect_uri.eq(&url));
    }

    fn get_potential_user(&self, username: &String, password: &String) -> AuthResult<User> {
        let potential_user = self.user_handler.get_by_username(username);
        match potential_user {
            Err(e) if e == DieselError::NotFound => Err(AuthError::NotFound),
            Ok(user) => {
                if verify(&user.password, password, &user.salt)? {
                    Ok(user)
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
        let token_bytes = self.decrypt(encrypted_token)?;

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

        let encrypted_token = self.encrypt(&token_bytes)?;
        Ok(encrypted_token)
    }

    fn generate_refresh_token(&self, _username: &String) -> AuthResult<RefreshToken> {
        let refresh_token = thread_rng().sample_iter(&Alphanumeric).take(60).collect();

        Ok(refresh_token)
    }

    fn generate_auth_code(&self, username: &String, client_id: &String) -> AuthResult<AuthCode> {
        let expiry_time = SystemTime::now()
            .add(Duration::new(self.auth_code_lifetime, 0))
            .duration_since(UNIX_EPOCH)?
            .as_millis();

        let token = AuthCodePayload {
            username: username.to_owned(),
            client_id: client_id.to_owned(),
            expiry_timestamp: expiry_time,
        };

        let token_bytes = serde_json::to_vec(&token)?;
        let encrypted_token = self.encrypt(&token_bytes)?;
        Ok(encrypted_token)
    }

    fn decrypt(&self, encrypted_token: &String) -> AuthResult<Vec<u8>> {
        let encrypted_token_bytes = base64::decode_config(encrypted_token, base64::URL_SAFE)?;
        let crypter = self.new_crypter();
        let mut crypter = panic::AssertUnwindSafe(crypter);
        let token_bytes = match panic::catch_unwind(move || crypter.decrypt_bytes_to_bytes(&encrypted_token_bytes)) {
            Ok(r) => r?,
            _ => {
                println!("[WIP] Recovering from magic crypt panic, nothing to see here");
                return Err(InvalidToken)
            },
        };
        Ok(token_bytes)
    }

    fn encrypt(&self, token_bytes: &Vec<u8>) -> AuthResult<String> {
        let mut crypter = self.new_crypter();
        let encrypted_token_bytes = crypter.encrypt_bytes_to_bytes(&token_bytes);
        let encrypted_token = base64::encode_config(&encrypted_token_bytes, base64::URL_SAFE);
        Ok(encrypted_token)
    }

    fn new_crypter(&self) -> MagicCrypt {
        new_magic_crypt!(self.cypher_key.clone(), 256)
    }
}

fn verify(key: &String, password: &String, salt: &String) -> AuthResult<bool> {
    Ok(bcrypt::verify(password.to_owned() + salt, key)?)
}

fn generate_password(password: &String, salt: &String) -> AuthResult<String> {
    Ok(bcrypt::hash(password.to_owned() + salt, 12)?)
}

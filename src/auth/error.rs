use std::error::Error;
use std::time::SystemTimeError;
use std::{convert, error, fmt};

use crate::auth::error::AuthError::{InternalError, NotFound, UserAlreadyExist};
use crate::database::handler::DbError;
use base64::DecodeError;
use diesel::result::Error as DieselError;
use serde_json;

#[derive(Debug)]
pub enum AuthError {
    NotFound,
    NotActivated,
    WrongPassword,
    InvalidToken,
    ExpiredToken,
    InvalidRedirectUri,
    InvalidClientID,
    UserAlreadyExist,
    UserAlreadyActivated,
    BcryptError(bcrypt::BcryptError),
    DBError(DieselError),
    JSONError(serde_json::Error),
    InternalError(Option<Box<dyn Error>>),
}

impl convert::From<DbError> for AuthError {
    fn from(e: DbError) -> AuthError {
        match e {
            DbError::DuplicateKey => UserAlreadyExist,
            DbError::NotFound => NotFound,
            DbError::InternalError(e) => InternalError(e),
        }
    }
}

impl convert::From<bcrypt::BcryptError> for AuthError {
    fn from(e: bcrypt::BcryptError) -> AuthError {
        AuthError::BcryptError(e)
    }
}

impl convert::From<DieselError> for AuthError {
    fn from(e: DieselError) -> AuthError {
        AuthError::DBError(e)
    }
}

impl convert::From<serde_json::Error> for AuthError {
    fn from(e: serde_json::Error) -> AuthError {
        AuthError::JSONError(e)
    }
}

impl convert::From<DecodeError> for AuthError {
    fn from(_: DecodeError) -> AuthError {
        AuthError::InvalidToken
    }
}

impl convert::From<SystemTimeError> for AuthError {
    fn from(e: SystemTimeError) -> AuthError {
        AuthError::InternalError(Some(Box::new(e)))
    }
}

impl convert::From<magic_crypt::Error> for AuthError {
    fn from(_: magic_crypt::Error) -> AuthError {
        AuthError::InvalidToken
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthError::NotFound => write!(f, "User not found"),
            AuthError::WrongPassword => write!(f, "Wrong password"),
            AuthError::InvalidToken => write!(f, "Invalid token"),
            AuthError::ExpiredToken => write!(f, "Expired token"),
            AuthError::InvalidRedirectUri => write!(f, "Invalid redirect uri"),
            AuthError::InvalidClientID => write!(f, "Invalid client id"),
            AuthError::NotActivated => write!(f, "Not activated"),
            AuthError::UserAlreadyExist => write!(f, "User/Email already exist"),
            AuthError::BcryptError(e) => write!(f, "BcryptError {}", e),
            AuthError::DBError(e) => write!(f, "DBError {}", e),
            AuthError::JSONError(e) => write!(f, "JSONError {}", e),
            AuthError::InternalError(e) => write!(f, "InternalError {:?}", e),
            AuthError::UserAlreadyActivated => write!(f, "User already activated"),
        }
    }
}

impl convert::From<AuthError> for actix_web::Error {
    fn from(e: AuthError) -> actix_web::Error {
        match e {
            AuthError::NotFound => actix_web::error::ErrorNotFound(e),
            AuthError::WrongPassword => actix_web::error::ErrorBadRequest(e),
            AuthError::InvalidToken => actix_web::error::ErrorBadRequest(e),
            AuthError::InvalidRedirectUri => actix_web::error::ErrorBadRequest(e),
            AuthError::InvalidClientID => actix_web::error::ErrorBadRequest(e),
            AuthError::UserAlreadyExist => actix_web::error::ErrorBadRequest(e),
            AuthError::ExpiredToken => actix_web::error::ErrorUnauthorized(e),
            AuthError::NotActivated => actix_web::error::ErrorUnauthorized(e),
            AuthError::UserAlreadyActivated => actix_web::error::ErrorBadRequest(e),
            _ => actix_web::error::ErrorInternalServerError(e),
        }
    }
}

impl error::Error for AuthError {}

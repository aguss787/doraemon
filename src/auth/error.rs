use std::{convert, error, fmt};
use std::error::Error;
use std::time::SystemTimeError;

use base64::DecodeError;
use diesel::result::Error as DieselError;
use serde_json;

#[derive(Debug)]
pub enum AuthError {
    NotFound,
    WrongPassword,
    InvalidToken,
    ExpiredToken,
    UserAlreadyExist,
    BcryptError(bcrypt::BcryptError),
    DBError(DieselError),
    JSONError(serde_json::Error),
    InternalError(Box<dyn Error>),
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
        AuthError::InternalError(Box::new(e))
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
            AuthError::UserAlreadyExist => write!(f, "User already exist"),
            AuthError::BcryptError(e) => write!(f, "BcryptError {}", e),
            AuthError::DBError(e) => write!(f, "DBError {}", e),
            AuthError::JSONError(e) => write!(f, "JSONError {}", e),
            AuthError::InternalError(e) => write!(f, "InternalError {}", e),
        }
    }
}

impl convert::From<AuthError> for actix_web::Error {
    fn from(e: AuthError) -> actix_web::Error {
        match e {
            AuthError::NotFound => actix_web::error::ErrorNotFound(e),
            AuthError::WrongPassword => actix_web::error::ErrorBadRequest(e),
            AuthError::InvalidToken => actix_web::error::ErrorBadRequest(e),
            AuthError::UserAlreadyExist => actix_web::error::ErrorBadRequest(e),
            AuthError::ExpiredToken => actix_web::error::ErrorUnauthorized(e),
            _ => actix_web::error::ErrorInternalServerError(e),
        }
    }
}

impl error::Error for AuthError {}

use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde::export::Formatter;
use std::error::Error;
use std::fmt;

pub mod client_credential;
pub mod url;
pub mod user;

pub type DbResult<T> = Result<T, DbError>;

#[derive(Debug)]
pub enum DbError {
    NotFound,
    DuplicateKey,
    InternalError(Option<Box<dyn Error>>),
}

impl From<DieselError> for DbError {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => DbError::NotFound,
            DieselError::InvalidCString(_) => DbError::InternalError(Option::Some(Box::new(e))),
            DieselError::DatabaseError(de, _) => match de {
                DatabaseErrorKind::UniqueViolation => DbError::DuplicateKey,
                _ => DbError::InternalError(Option::Some(Box::new(e))),
            },
            DieselError::QueryBuilderError(_) => DbError::InternalError(Option::Some(Box::new(e))),
            DieselError::DeserializationError(_) => {
                DbError::InternalError(Option::Some(Box::new(e)))
            }
            DieselError::SerializationError(_) => DbError::InternalError(Option::Some(Box::new(e))),
            DieselError::RollbackTransaction => DbError::InternalError(Option::Some(Box::new(e))),
            DieselError::AlreadyInTransaction => DbError::InternalError(Option::Some(Box::new(e))),
            DieselError::__Nonexhaustive => DbError::InternalError(Option::Some(Box::new(e))),
        }
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DbError::NotFound => write!(f, "Not Found"),
            DbError::DuplicateKey => write!(f, "Duplicate Key"),
            DbError::InternalError(e) => write!(f, "Internal Error {:?}", e),
        }
    }
}

impl From<DbError> for actix_web::Error {
    fn from(e: DbError) -> Self {
        match e {
            DbError::NotFound => actix_web::error::ErrorNotFound(e),
            DbError::DuplicateKey => actix_web::error::ErrorConflict(e),
            DbError::InternalError(_) => actix_web::error::ErrorInternalServerError(e),
        }
    }
}

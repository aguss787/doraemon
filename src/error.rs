use std::fmt;

#[derive(Debug)]
pub enum Error {
    MailError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MailError => write!(f, "Error getting mail service"),
        }
    }
}

impl From<Error> for actix_web::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::MailError => actix_web::error::ErrorInternalServerError(e),
        }
    }
}

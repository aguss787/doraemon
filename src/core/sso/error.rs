use std::fmt;

#[derive(Debug)]
pub enum SsoError {
    CookieNotFound,
}

impl fmt::Display for SsoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SsoError::CookieNotFound => write!(f, "Bad boi!"),
        }
    }
}

impl From<SsoError> for actix_web::Error {
    fn from(e: SsoError) -> Self {
        match e {
            SsoError::CookieNotFound => actix_web::error::ErrorBadRequest(e),
        }
    }
}

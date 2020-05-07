use std::fmt;

#[derive(Debug)]
pub enum TemplaterError {
    RenderError,
}

pub type TemplateResult<T> = Result<T, TemplaterError>;

impl fmt::Display for TemplaterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplaterError::RenderError => write!(f, "Error during rendering template"),
        }
    }
}

impl From<TemplaterError> for actix_web::Error {
    fn from(e: TemplaterError) -> Self {
        match e {
            TemplaterError::RenderError => actix_web::error::ErrorInternalServerError(e),
        }
    }
}

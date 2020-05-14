use crate::templater::error::TemplateResult;

pub mod error;
pub mod tera_based;

pub trait Templater {
    fn login_page(&self, client_id: &String, redirect_uri: &String) -> TemplateResult<String>;
    fn register_page(&self) -> TemplateResult<String>;
    fn resend_activation_page(&self, message: &String) -> TemplateResult<String>;
}

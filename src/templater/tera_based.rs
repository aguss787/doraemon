use serde::Serialize;
use tera::{Context, Tera};

use crate::templater::error::TemplaterError::RenderError;
use crate::templater::error::{TemplateResult, TemplaterError};
use crate::templater::Templater;

pub struct TeraTemplater {
    tera: Tera,
}

impl From<tera::Error> for TemplaterError {
    fn from(_: tera::Error) -> Self {
        RenderError
    }
}

impl TeraTemplater {
    pub fn new(tera: Tera) -> TeraTemplater {
        TeraTemplater { tera }
    }

    fn render<T: Serialize>(&self, template: &str, payload: Option<&T>) -> TemplateResult<String> {
        let mut context = Context::new();

        if payload.is_some() {
            context.insert("payload", payload.unwrap());
        }

        Ok(self.tera.render(template, &context)?)
    }
}

impl Templater for TeraTemplater {
    fn login_page(&self, client_id: &String, redirect_uri: &String) -> TemplateResult<String> {
        #[derive(Serialize)]
        struct Payload<'a> {
            client_id: &'a String,
            redirect_uri: &'a String,
        }

        self.render::<Payload>(
            "account/login.html",
            Some(&Payload {
                client_id,
                redirect_uri,
            }),
        )
    }

    fn register_page(&self) -> TemplateResult<String> {
        self.render::<()>(
            "account/register.html",
            None,
        )
    }
}

use diesel::PgConnection;
use tera::Tera;

use crate::auth::{Auth, AuthHandler};
use crate::config::Config;
use crate::database::handler::client_credential::ClientCredentialPostgresHandler;
use crate::database::handler::url::{UrlHandler, UrlPostgresHandler};
use crate::database::handler::user::UserPostgresHandler;
use crate::error::Error;
use crate::templater::tera_based::TeraTemplater;
use crate::templater::Templater;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, SmtpTransport};
use std::rc::Rc;

pub struct AppData {
    pub connection: Rc<PgConnection>,
    pub auth_handler: Rc<dyn AuthHandler>,
    pub url_handler: Rc<dyn UrlHandler>,
    pub templater: Box<dyn Templater>,
    pub config: Config,
}

impl AppData {
    pub fn new(connection: PgConnection, config: &Config) -> AppData {
        let tera = Tera::new("src/templates/**/*.html").expect("Missing template");
        let connection = Rc::new(connection);

        let client_credential_handler =
            Rc::new(ClientCredentialPostgresHandler::new(connection.clone()));
        let user_handler = Rc::new(UserPostgresHandler::new(connection.clone()));
        let url_handler = Rc::new(UrlPostgresHandler::new(connection.clone()));

        let auth_handler = Rc::new(Auth::new(
            config.auth.cypher_key.clone(),
            config.auth.token_lifetime,
            config.auth.auth_code_lifetime,
            config.auth.activation_code_lifetime,
            user_handler.clone(),
            client_credential_handler.clone(),
        ));

        AppData {
            connection: connection.clone(),
            auth_handler,
            url_handler,
            templater: Box::new(TeraTemplater::new(tera)),
            config: config.clone(),
        }
    }

    pub fn mailer(&self) -> Result<SmtpTransport, Error> {
        let creds = Credentials::new(
            self.config.gmail.username.to_owned(),
            self.config.gmail.password.to_owned(),
        );

        let client = SmtpClient::new_simple(self.config.gmail.smtp_host.as_str())
            .map_err(|_| Error::MailError)?;

        Ok(client.credentials(creds).transport())
    }
}

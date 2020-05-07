use diesel::PgConnection;
use tera::Tera;

use crate::auth::Auth;
use crate::config::Config;
use crate::database::handler::client_credential::ClientCredentialHandler;
use crate::database::handler::url::UrlHandler;
use crate::database::handler::user::UserHandler;
use crate::templater::Templater;
use crate::templater::tera_based::TeraTemplater;

pub struct AppData {
    pub connection: PgConnection,
    pub templater: Box<dyn Templater>,
    config: Config,
}

impl AppData {
    pub fn new(connection: PgConnection, config: &Config) -> AppData {
        let tera = Tera::new("src/templates/**/*.html").expect("Missing template");

        AppData {
            connection,
            templater: Box::new(TeraTemplater::new(tera)),
            config: config.clone(),
        }
    }

    pub fn auth(&self) -> Auth {
        Auth::new(
            &self.config.auth.cypher_key,
            self.config.auth.token_lifetime,
            self.config.auth.auth_code_lifetime,
            self.user_handler(),
            self.client_credential_handler(),
        )
    }

    pub fn user_handler(&self) -> UserHandler {
        UserHandler::new(&self.connection)
    }

    pub fn url_handler(&self) -> UrlHandler {
        UrlHandler::new(&self.connection)
    }

    pub fn client_credential_handler(&self) -> ClientCredentialHandler {
        ClientCredentialHandler::new(&self.connection)
    }
}

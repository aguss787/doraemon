use diesel::PgConnection;

use crate::auth::Auth;
use crate::config::Config;
use crate::database::handler::url::UrlHandler;
use crate::database::handler::user::UserHandler;

pub struct AppData {
    pub connection: PgConnection,
    config: Config,
}

impl AppData {
    pub fn new(connection: PgConnection, config: &Config) -> AppData {
        AppData {
            connection,
            config: config.clone(),
        }
    }

    pub fn auth(&self) -> Auth {
        Auth::new(
            &self.config.auth.cypher_key,
            self.config.auth.token_lifetime,
            self.user_handler(),
        )
    }

    pub fn user_handler(&self) -> UserHandler {
        UserHandler::new(&self.connection)
    }

    pub fn url_handler(&self) -> UrlHandler {
        UrlHandler::new(&self.connection)
    }
}

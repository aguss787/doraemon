use diesel::pg::PgConnection;
use diesel::prelude::Connection;

use crate::config::Config;

pub mod handler;

pub fn establish_connection(config: &Config) -> PgConnection {
    let database_config = &config.database;
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database_config.user,
        database_config.password,
        database_config.host,
        database_config.port,
        database_config.database
    );
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

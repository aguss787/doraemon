use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl, TextExpressionMethods};

use crate::schema::client_credential::dsl as client_credential;
use std::rc::Rc;

pub trait ClientCredentialHandler {
    fn get_by_id(&self, id: &String) -> QueryResult<ClientCredential>;
}

#[derive(Queryable)]
pub struct ClientCredential {
    pub id: String,
    pub secret: String,
    pub redirect_uri: String,
}

pub struct ClientCredentialPostgresHandler {
    pub connection: Rc<PgConnection>,
}

impl ClientCredentialPostgresHandler {
    pub fn new(connection: Rc<PgConnection>) -> ClientCredentialPostgresHandler {
        ClientCredentialPostgresHandler { connection }
    }
}

impl ClientCredentialHandler for ClientCredentialPostgresHandler {
    fn get_by_id(&self, id: &String) -> QueryResult<ClientCredential> {
        client_credential::client_credential
            .filter(client_credential::id.like(id))
            .first::<ClientCredential>(self.connection.as_ref())
    }
}

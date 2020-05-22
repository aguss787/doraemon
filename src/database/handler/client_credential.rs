use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl, TextExpressionMethods};

use crate::schema::client_credential::dsl as client_credential;

#[derive(Queryable)]
pub struct ClientCredential {
    pub id: String,
    pub secret: String,
    pub redirect_uri: String,
}

pub struct ClientCredentialHandler<'a> {
    pub connection: &'a PgConnection,
}

impl<'a> ClientCredentialHandler<'a> {
    pub fn new(connection: &'a PgConnection) -> ClientCredentialHandler {
        ClientCredentialHandler { connection }
    }
}

impl<'a> ClientCredentialHandler<'a> {
    pub fn get_by_id(&self, id: &String) -> QueryResult<ClientCredential> {
        client_credential::client_credential
            .filter(client_credential::id.like(id))
            .first::<ClientCredential>(self.connection)
    }
}

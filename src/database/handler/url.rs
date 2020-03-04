use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl, TextExpressionMethods};

use crate::schema::url::dsl::*;

#[derive(Queryable)]
pub struct Url {
    pub _key: String,
    pub target: String,
}

pub struct UrlHandler<'a> {
    pub connection: &'a PgConnection,
}

impl<'a> UrlHandler<'a> {
    pub fn new(connection: &'a PgConnection) -> UrlHandler {
        UrlHandler { connection }
    }
}

impl<'a> UrlHandler<'a> {
    pub fn get_by_key(&self, url_key: &String) -> QueryResult<Url> {
        url.filter(key.like(url_key)).first::<Url>(self.connection)
    }
}

use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl, TextExpressionMethods};

use crate::schema::url::dsl::*;

#[derive(Queryable)]
pub struct Url {
    pub _key: String,
    pub target: String,
}

pub fn get_by_key(connection: &PgConnection, url_key: &String) -> QueryResult<Url> {
    url.filter(key.like(url_key)).first::<Url>(connection)
}

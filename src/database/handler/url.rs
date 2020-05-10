use diesel::{delete, insert_into, update, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::Serialize;

use crate::database::handler::{DbError, DbResult};
use crate::schema::url as url_schema;
use crate::schema::url::dsl as url_dsl;

#[derive(Queryable, Serialize)]
pub struct Url {
    pub key: String,
    pub target: String,
    pub username: String,
}

pub struct UrlHandler<'a> {
    pub connection: &'a PgConnection,
}

impl<'a> UrlHandler<'a> {
    pub fn new(connection: &'a PgConnection) -> UrlHandler {
        UrlHandler { connection }
    }
}

#[derive(Insertable)]
#[table_name = "url_schema"]
pub struct NewUser<'a> {
    pub key: &'a String,
    pub target: &'a String,
    pub username: &'a String,
}

impl<'a> UrlHandler<'a> {
    pub fn get_by_key(&self, url_key: &String) -> DbResult<Url> {
        Ok(url_dsl::url
            .filter(url_dsl::key.eq(url_key))
            .first::<Url>(self.connection)?)
    }

    pub fn get_by_username(
        &self,
        username: &String,
        offset: i64,
        limit: i64,
    ) -> DbResult<Vec<Url>> {
        Ok(url_dsl::url
            .filter(url_dsl::username.eq(username))
            .order(url_dsl::key)
            .offset(offset)
            .limit(limit)
            .load::<Url>(self.connection)?)
    }

    pub fn insert(&self, key: &String, target: &String, username: &String) -> DbResult<()> {
        let new_user = NewUser {
            key,
            target,
            username,
        };
        insert_into(url_dsl::url)
            .values(new_user)
            .execute(self.connection)?;
        Ok(())
    }

    pub fn delete(&self, key: &String, username: &String) -> DbResult<usize> {
        let count = delete(
            url_dsl::url
                .filter(url_dsl::key.eq(key))
                .filter(url_dsl::username.eq(username)),
        )
        .execute(self.connection)?;
        Ok(count)
    }

    pub fn delete_at_least_one(&self, key: &String, username: &String) -> DbResult<usize> {
        let count = self.delete(key, username)?;
        if count == 0 {
            Err(DbError::NotFound)
        } else {
            Ok(count)
        }
    }

    pub fn update(
        &self,
        old_key: &String,
        username: &String,
        new_key: &String,
        target: &String,
    ) -> DbResult<Url> {
        let result = update(
            url_dsl::url
                .filter(url_dsl::key.eq(old_key))
                .filter(url_dsl::username.eq(username)),
        )
        .set((url_dsl::key.eq(new_key), url_dsl::target.eq(target)))
        .get_result::<Url>(self.connection)?;

        Ok(result)
    }
}

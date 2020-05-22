use diesel::{delete, insert_into, update, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::Serialize;

use crate::database::handler::{DbError, DbResult};
use crate::schema::url as url_schema;
use crate::schema::url::dsl as url;

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
        Ok(url::url
            .filter(url::key.eq(url_key))
            .first::<Url>(self.connection)?)
    }

    pub fn get_by_username(
        &self,
        username: &String,
        offset: i64,
        limit: i64,
    ) -> DbResult<Vec<Url>> {
        Ok(url::url
            .filter(url::username.eq(username))
            .order(url::key)
            .offset(offset)
            .limit(limit)
            .load::<Url>(self.connection)?)
    }

    pub fn count_by_username(&self, username: &String) -> DbResult<i64> {
        Ok(url::url
            .filter(url::username.eq(username))
            .count()
            .first::<i64>(self.connection)?)
    }

    pub fn get_by_key_and_username(&self, key: &String, username: &String) -> DbResult<Url> {
        Ok(url::url
            .filter(url::key.eq(key))
            .filter(url::username.eq(username))
            .first::<Url>(self.connection)?)
    }

    pub fn insert(&self, key: &String, target: &String, username: &String) -> DbResult<()> {
        let new_user = NewUser {
            key,
            target,
            username,
        };
        insert_into(url::url)
            .values(new_user)
            .execute(self.connection)?;
        Ok(())
    }

    pub fn delete(&self, key: &String, username: &String) -> DbResult<usize> {
        let count = delete(
            url::url
                .filter(url::key.eq(key))
                .filter(url::username.eq(username)),
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
            url::url
                .filter(url::key.eq(old_key))
                .filter(url::username.eq(username)),
        )
        .set((url::key.eq(new_key), url::target.eq(target)))
        .get_result::<Url>(self.connection)?;

        Ok(result)
    }
}

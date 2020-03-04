use diesel::{
    insert_into, PgConnection, QueryDsl, QueryResult, RunQueryDsl, TextExpressionMethods,
};

use crate::schema::user as user_schema;
use crate::schema::user::dsl as user_dsl;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: String,
}

#[derive(Insertable)]
#[table_name = "user_schema"]
pub struct NewUser<'a> {
    pub username: &'a String,
    pub password: &'a String,
    pub salt: &'a String,
}

pub struct UserHandler<'a> {
    pub connection: &'a PgConnection,
}

impl<'a> UserHandler<'a> {
    pub fn new(connection: &'a PgConnection) -> UserHandler {
        UserHandler { connection }
    }
}

impl<'a> UserHandler<'a> {
    pub fn new_user(&self, new_user: &NewUser) -> QueryResult<()> {
        insert_into(user_dsl::user)
            .values(new_user)
            .execute(self.connection)?;
        Ok(())
    }

    pub fn get_by_username(&self, username: &String) -> QueryResult<User> {
        user_dsl::user
            .filter(user_dsl::username.like(username))
            .first::<User>(self.connection)
    }
}

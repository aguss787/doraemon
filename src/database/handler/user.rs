use diesel::{
    insert_into, update, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    TextExpressionMethods,
};

use crate::database::handler::DbResult;
use crate::schema::user as user_schema;
use crate::schema::user::dsl as user;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: String,
    pub is_activated: bool,
}

#[derive(Insertable)]
#[table_name = "user_schema"]
pub struct NewUser<'a> {
    pub username: &'a String,
    pub email: &'a String,
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
    pub fn new_user(&self, new_user: &NewUser) -> DbResult<()> {
        insert_into(user::user)
            .values(new_user)
            .execute(self.connection)?;
        Ok(())
    }

    pub fn get_by_username(&self, username: &String) -> DbResult<User> {
        Ok(user::user
            .filter(user::username.like(username))
            .first::<User>(self.connection)?)
    }

    pub fn activate_by_username(&self, username: &String) -> DbResult<usize> {
        let result = update(user::user.filter(user::username.eq(username)))
            .set(user::is_activated.eq(true))
            .execute(self.connection)?;

        Ok(result)
    }
}

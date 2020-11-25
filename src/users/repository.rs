#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::users;
use crate::users::User;
use rocket::http::{Cookies, Cookie};



pub fn register_session(mut cookies: Cookies) {
    cookies.add_private(Cookie::new("user_id_session", "value"));

}

pub fn validate(user: InsertableUser, connection: &PgConnection) -> QueryResult<bool> {
    match users::table.filter(
        users::name.eq(user.name)
    ).filter(
        users::password.eq(user.password)
    ).get_results::<User>(&*connection) {
        Ok(users) => {
            if users.len() == 1 { return Ok(true) }
            else { return Ok(false) }
        },
        Err(e) => return Err(e),
    }
}

pub fn insert(user: InsertableUser, connection: &PgConnection) -> QueryResult<bool> {
    match diesel::insert_into(users::table)
        .values(&user)
        .get_result::<User>(connection) {
            Ok(users) => return Ok(true),
            Err(e) => return Err(e)
        }
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct InsertableUser {
    name: String,
    password: String
}

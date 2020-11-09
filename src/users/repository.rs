#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::users;
use crate::users::User;

    pub fn post(user: QueryableUser, connection: &PgConnection) -> QueryResult<bool> {
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

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct QueryableUser {
    name: String,
    password: String
}

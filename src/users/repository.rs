#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::users;
use crate::users::User;

pub fn post(user: User, connection: &PgConnection) -> QueryResult<bool> {
    let query_result = users::table.select(
        (users::name.eq(user.name), users::password.eq(user.password))
    ).get_results::<Vec<User>>(connection);

    let result = match query_result {
        Ok(users) => {
            if users.len() == 1 { return Ok(true) }
            else { return Ok(false) }
        },
        Err(e) => return Err(e),
    };
}

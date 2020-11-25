#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::sessions;
use crate::sessions::Session;
use rocket::http::{Cookies, Cookie};
use rand::Rng;
use rand::distributions::Alphanumeric;
use diesel::result::Error;

pub fn generate_random_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>()
}

pub fn register_session(mut cookies: Cookies, connection: &PgConnection) {
    let secrect_key = generate_random_key();
    insert(&secrect_key, &connection);
    cookies.add_private(Cookie::new("user_id_session", secrect_key));
}

pub fn end_session(mut cookies: Cookies, connection: &PgConnection) -> QueryResult<bool> {
    cookies.remove_private(Cookie::named("user_id_session"));
    return Ok(true)
}

pub fn insert(secrect_key: &String, connection: &PgConnection) -> QueryResult<bool> {
    let newSessions = InsertableSession{sessionid:secrect_key.to_string()};
    match diesel::insert_into(sessions::table)
        .values(&newSessions)
        .get_result::<Session>(connection) {
            Ok(users) => return Ok(true),
            Err(e) => return Err(e)
        }
}

//pub fn delete(connection: &PgConnection) -> QueryResult<usize> {
//    diesel::delete(sessions::table.find(id))
//        .execute(connection)
//}

pub fn validate_session(mut cookies: Cookies, connection: &PgConnection) -> QueryResult<bool> {
    println!("hi there {:?}", cookies.get_private("user_id_session"));
    match cookies.get_private("user_id_session") {
        Some(cookie) => {
            match sessions::table.filter(
                    sessions::sessionid.eq(cookie.value().to_string())
                ).get_results::<Session>(&*connection) {
                    Ok(sessions) => {
                        println!("checking for session OKOK");
                        if sessions.len() == 1 {
                            return Ok(true)
                        }
                        else { return Ok(false) }
                    },
                    Err(e) => return Err(e),
            }
        },
        None => return Err(Error::NotFound)
    }
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Session> {
    sessions::table.find(id).get_result::<Session>(connection)
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "sessions"]
pub struct InsertableSession {
    pub sessionid: String
}

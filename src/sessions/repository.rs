#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::sessions;
use crate::sessions::Session;

pub fn insert(session: InsertableSession, connection: &PgConnection) -> QueryResult<Session> {
    diesel::insert_into(sessions::table)
        .values(&session)
        .get_result(connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Session> {
    sessions::table.find(id).get_result::<Session>(connection)
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "sessions"]
pub struct InsertableSession {
    pub sessionid: String
}

#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::events;
use crate::events::Event;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Event>> {
    events::table.load::<Event>(&*connection)
}

pub fn insert(event: InsertableEvent, connection: &PgConnection) -> QueryResult<Event> {
    diesel::insert_into(events::table)
        .values(&event)
        .get_result(connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Event> {
    events::table.find(id).get_result::<Event>(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(events::table.find(id))
        .execute(connection)
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "events"]
pub struct InsertableEvent {
    name: String,
    date: String,
    location: String,
}

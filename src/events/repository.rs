#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::events;
use crate::events::Event;

pub fn insert(event: Event, connection: &PgConnection) -> QueryResult<Event> {
    diesel::insert_into(events::table)
        .values(&InsertableEvent::from_event(event))
        .get_result(connection)
}

#[derive(Insertable)]
#[table_name = "events"]
struct InsertableEvent {
    name: String,
    date: String,
    location: String,
}

impl InsertableEvent {

    fn from_event(event: Event) -> InsertableEvent {
        InsertableEvent {
            name: event.name,
            date: event.date,
            location: event.location
        }
    }
}

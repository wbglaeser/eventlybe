#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::events;

pub mod handler;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "events"]
pub struct Event {
    id: i32,
    name: String,
    date: String,
    location: String,
}

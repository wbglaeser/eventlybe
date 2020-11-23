#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::sessions;

pub mod handler;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "sessions"]
pub struct Session {
    id: i32,
    sessionid: String
}

#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::users;

pub mod handler;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    id: i32,
    pub name: String,
    pub password: String
}

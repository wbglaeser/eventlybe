use serde::{Serialize, Deserialize};
#[macro_use] extern crate rocket;

const EventDetails_LIMIT: u64 = 256;

#[derive(Serialize, Deserialize)]
pub struct EventDetails {
    pub name: String,
    date: String,
    location: String
}

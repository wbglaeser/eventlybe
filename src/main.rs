#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use backend::*;
use rocket_contrib::json::Json;

#[post("/", format="json", data = "<input>")]
fn new(input: Json<EventDetails>) -> String {
    input.name.to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![new]).launch();
}

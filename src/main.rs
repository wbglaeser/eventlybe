#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use backend::*;

#[post("/", data = "<input>")]
fn new(input: EventDetails) -> String {
    input.name.to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![new]).launch();
}

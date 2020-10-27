#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use backend::*;
use rocket_contrib::json::Json;

#[post("/", format="json", data = "<input>")]
fn new(input: Json<EventDetails>) -> String {
    input.name.to_string()
}

extern crate backend;
#[macro_use] extern crate diesel;

use self::backend::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use self::schema::events::dsl::*;

    let connection = establish_connection();
    let results = events.limit(5)
        .load::<Event>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} events", results.len());
    for event in results {
        println!("{}", event.name);
        println!("----------\n");
        println!("{}", event.date);
    }

    rocket::ignite().mount("/", routes![new]).launch();
}

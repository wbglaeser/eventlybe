#![feature(proc_macro_hygiene, decl_macro)]

use std::io::{self, Read};
use rocket::data::{FromData, Outcome, Transform, Transformed};
use rocket::http::Status;
use rocket::{Request, Data, Outcome::*};

#[macro_use] extern crate rocket;

#[derive(Debug, Clone)]
struct EventDetails<'a> {
    name: &'a str,
}

enum EventDetailsError {
    Io(io::Error),
    Parse
}

const EventDetails_LIMIT: u64 = 256;

impl<'a> FromData<'a> for EventDetails<'a> {
    type Error = EventDetailsError;
    type Owned = String;
    type Borrowed = str;

    fn transform(_: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        let mut stream = data.open().take(EventDetails_LIMIT);
        let mut string = String::with_capacity((EventDetails_LIMIT / 2) as usize);
        let outcome = match stream.read_to_string(&mut string) {
            Ok(_) => Success(string),
            Err(e) => Failure((Status::InternalServerError, EventDetailsError::Io(e)))
        };

        // Returning `Borrowed` here means we get `Borrowed` in `from_data`.
        Transform::Borrowed(outcome)
    }

    fn from_data(_: &Request, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error> {
        // Retrieve a borrow to the now transformed `String` (an &str). This
        // is only correct because we know we _always_ return a `Borrowed` from
        // `transform` above.
        let string = outcome.borrowed()?;

        // Perform a crude, inefficient parse.
        let splits: Vec<&str> = string.split(" ").collect();
        if splits.len() != 2 || splits.iter().any(|s| s.is_empty()) {
            return Failure((Status::UnprocessableEntity, EventDetailsError::Parse));
        }

        // Return successfully.
        Success(EventDetails { name: splits[0] })
    }
}

#[post("/", data = "<input>")]
fn new(input: EventDetails) -> String {
    input.name.to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![new]).launch();
}

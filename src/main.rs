#![feature(proc_macro_hygiene, decl_macro)]

use std::io::{self, Read};
use rocket::data::{FromData, Outcome, Transform, Transformed};
use rocket::http::Status;
use rocket::{Request, Data, Outcome::*};

#[macro_use] extern crate rocket;

struct Name<'a> {
    first: &'a str,
}

enum NameError {
    Io(io::Error),
    Parse
}

const NAME_LIMIT: u64 = 256;

impl<'a> FromData<'a> for Name<'a> {
    type Error = NameError;
    type Owned = String;
    type Borrowed = str;

    fn transform(_: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        let mut stream = data.open().take(NAME_LIMIT);
        let mut string = String::with_capacity((NAME_LIMIT / 2) as usize);
        let outcome = match stream.read_to_string(&mut string) {
            Ok(_) => Success(string),
            Err(e) => Failure((Status::InternalServerError, NameError::Io(e)))
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
        if splits.len() != 2  {
            return Failure((Status::UnprocessableEntity, NameError::Parse));
        }

        // Return successfully.
        Success(Name { first: splits[0] })
    }
}

#[post("/", data = "<input>")]
fn new(input: Name) -> &'static str {
    "hello there!"
}

fn main() {
    rocket::ignite().mount("/", routes![new]).launch();
}

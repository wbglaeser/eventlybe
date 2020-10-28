use crate::connection::DbConn;
use diesel::result::Error;
use std::env;
use crate::events;
use events::Event;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[post("/", format = "application/json", data = "<event>")]
pub fn post(event: Json<Event>, connection: DbConn) -> Result<status::Created<Json<Event>>, Status> {
    events::repository::insert(event.into_inner(), &connection)
        .map(|event| event_created(event))
        .map_err(|error| error_status(error))
}

fn event_created(event: Event) -> status::Created<Json<Event>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    status::Created(
        format!("{host}:{port}/evently/{id}", host = host, port = port, id = event.id).to_string(),
        Some(Json(event)))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

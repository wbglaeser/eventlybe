use crate::connection::DbConn;
use diesel::result::Error;
use std::env;
use crate::events;
use crate::sessions;
use events::Event;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket::http::{Cookies, Cookie};

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Event>>, Status> {
    events::repository::all(&connection)
        .map(|event| Json(event))
        .map_err(|error| error_status(error))
}

#[post("/", data = "<event>")]
pub fn post(event: Json<events::repository::InsertableEvent>, connection: DbConn, mut cookies: Cookies) -> Result<status::Created<Json<Event>>, Status> {
    sessions::repository::validate_session(cookies, &connection);
    events::repository::insert(event.into_inner(), &connection)
        .map(|event| event_created(event))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Event>, Status> {
    events::repository::get(id, &connection)
        .map(|event| Json(event))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match events::repository::get(id, &connection) {
        Ok(_) => events::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
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

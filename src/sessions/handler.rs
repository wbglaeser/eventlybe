use crate::connection::DbConn;
use diesel::result::Error;
use std::env;
use crate::sessions;
use sessions::Session;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket::http::{Cookies, Cookie};


#[post("/")]
pub fn post(connection: DbConn, mut cookies: Cookies) -> Result<status::Created<Json<Session>>, Status> {
    let cookie = cookies.get("name");
    println!("{:?}", cookie);
    let session = sessions::repository::InsertableSession{sessionid: String::from("test")};
    sessions::repository::insert(session, &connection)
        .map(|session| session_created(session))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Session>, Status> {
    sessions::repository::get(id, &connection)
        .map(|session| Json(session))
        .map_err(|error| error_status(error))
}

fn session_created(session: Session) -> status::Created<Json<Session>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    status::Created(
        format!("{host}:{port}/evently/{id}", host = host, port = port, id = session.id).to_string(),
        Some(Json(session)))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

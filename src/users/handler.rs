use crate::connection::DbConn;
use diesel::result::Error;
use crate::users;
use users::User;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket::response::status;
use std::env;
use rocket::http::{Cookies, Cookie};
use crate::sessions;

#[post("/validate_session")]
pub fn validate_session(connection: DbConn, mut cookies: Cookies) -> Result<Json<bool>, Status> {
    sessions::repository::validate_session(cookies, &connection)
        .map(|resp| Json(resp))
        .map_err(|error| error_status(error))
}

#[post("/login", data = "<user>")]
pub fn login(user: Json<users::repository::InsertableUser>, connection: DbConn, mut cookies: Cookies) -> Result<Json<bool>, Status> {
    users::repository::validate(user.into_inner(), &connection)
        .map(|auth| {
            sessions::repository::register_session(cookies, &connection);
            Json(auth)
        })
        .map_err(|error| error_status(error))
}

#[post("/logout")]
pub fn logout(connection: DbConn, mut cookies: Cookies) -> Result<Json<bool>, Status> {
    sessions::repository::end_session(cookies, &connection)
        .map(|resp| Json(resp))
        .map_err(|error| error_status(error))
}

#[post("/register", data = "<user>")]
pub fn register(user: Json<users::repository::InsertableUser>, connection: DbConn) ->  Result<Json<bool>, Status> {
    users::repository::insert(user.into_inner(), &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

fn user_created(user: User) -> status::Created<Json<User>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    status::Created(
        format!("{host}:{port}/evently/{id}", host = host, port = port, id = user.id).to_string(),
        Some(Json(user)))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

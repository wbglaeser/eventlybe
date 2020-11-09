use crate::connection::DbConn;
use diesel::result::Error;
use crate::users;
use users::User;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/", data = "<user>")]
pub fn post(user: Json<users::repository::QueryableUser>, connection: DbConn) -> Result<Json<bool>, Status> {
    users::repository::post(user.into_inner(), &connection)
        .map(|auth| Json(auth))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

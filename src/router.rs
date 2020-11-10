use crate::events;
use crate::users;
use rocket;
use crate::connection;
use crate::cors;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .attach(cors::CORS())
        .mount("/events",
               routes![
               events::handler::post,
               events::handler::get,
               events::handler::all,
               events::handler::delete],
        ).mount("/users",
               routes![
               users::handler::validate,
               users::handler::register],
        ).launch();
}

use crate::events;
use rocket;
use crate::connection;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .attach(events::cors::CORS())
        .mount("/events",
               routes![
               events::handler::post,
               events::handler::get,
               events::handler::all,
               events::handler::delete],
        ).launch();
}

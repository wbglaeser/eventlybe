use crate::events;
use rocket;
use crate::connection;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/events",
               routes![events::handler::post],
        ).launch();
}

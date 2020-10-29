use crate::users;
use rocket;
use crate::connection;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .attach(users::cors::CORS())
        .mount("/users",
               routes![
               users::handler::post],
        ).launch();
}

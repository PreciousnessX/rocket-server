use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod routes;

trait ServerProcess {
    fn add_routes(self) -> Rocket<Build>;
}

impl ServerProcess for Rocket<Build> {
    fn add_routes(self) -> Self {
        routes::add_routes(self)
    }
}

#[launch]
pub fn rocket() -> Rocket<Build> {
    rocket::build().add_routes()
}

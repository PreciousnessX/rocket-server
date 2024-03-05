use rocket::{Build, Rocket};

mod rocket_tutorial;
mod user;

pub fn add_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount("/rocket/tutorial", rocket_tutorial::get_routes())
        .mount("/api/user", user::get_routes())
}

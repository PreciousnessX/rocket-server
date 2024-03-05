use rocket_server;
#[rocket::main]
async fn main() {
    if let Err(e) = rocket_server::rocket().launch().await {
        eprintln!("Rocket launch failed: {:?}", e);
    }
}

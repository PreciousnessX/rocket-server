use rocket::get;
use rocket::tokio::time::{sleep, Duration};
use std::io;

// 注册
#[get("/register")]
fn register() -> &'static str {
    "register"
}

// 登录
#[get("/login")]
fn login() -> &'static str {
    "login"
}

// 退出
#[get("/logout")]
fn logout() -> &'static str {
    "logout"
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![register, login, logout]
}

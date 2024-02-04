use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Form, Router};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use crate::{web::AUTH_TOKEN, AppState};

pub fn routes() -> Router<AppState> {
    Router::new().route("/login", get(login_handler).post(api_login))
}

#[derive(Template)]
#[template(path = "routes/login/login.html")]
pub struct LoginTemplate;

pub async fn login_handler() -> impl IntoResponse {
    println!("->> {:<12} - login_handler", "HANDLER");
    LoginTemplate
}

#[derive(Template)]
#[template(path = "routes/login/login_fail.html")]
pub struct LoginFailTemplate;

#[derive(Template)]
#[template(path = "routes/login/login_success.html")]
pub struct LoginSuccessTemplate;

async fn api_login(cookies: Cookies, Form(payload): Form<LoginPayload>) -> impl IntoResponse {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic

    if payload.username != "demo1" || payload.password != "welcome" {
        LoginFailTemplate.into_response()
    } else {
        // TODO: Implement a real auth-token generation/signature
        cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

        LoginSuccessTemplate.into_response()
    }
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

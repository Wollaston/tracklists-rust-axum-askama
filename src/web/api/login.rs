use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Form, Router};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use crate::{web::AUTH_TOKEN, AppState};

pub fn routes() -> Router<AppState> {
    Router::new().route("/login", get(login_handler).post(api_login))
}

#[derive(Template)]
#[template(path = "routes/login/login.html")]
pub struct LoginTemplate;

pub async fn login_handler() -> impl IntoResponse {
    debug!("{:<12} - login_handler", "HANDLER");
    LoginTemplate
}

#[derive(Template)]
#[template(path = "routes/login/login_fail.html")]
pub struct LoginFailTemplate;

#[derive(Template)]
#[template(path = "routes/login/login_success.html")]
pub struct LoginSuccessTemplate;

async fn api_login(cookies: Cookies, Form(payload): Form<LoginPayload>) -> impl IntoResponse {
    debug!("{:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic

    if payload.username != "demo1" || payload.password != "welcome" {
        (http::StatusCode::INTERNAL_SERVER_ERROR, LoginFailTemplate).into_response()
    } else {
        // TODO: Implement a real auth-token generation/signature
        let user_id = uuid::Uuid::new_v4();
        cookies.add(Cookie::new(AUTH_TOKEN, format!("{}.exp.sign", user_id)));
        debug!("{:?}", cookies.get(AUTH_TOKEN));

        LoginSuccessTemplate.into_response()
    }
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

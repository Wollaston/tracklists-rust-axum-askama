use crate::{Error, Result};
use axum::response::Response;
use axum::{body::Body, extract::Request, middleware::Next};
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth(
    cookies: Cookies,
    request: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(request).await)
}

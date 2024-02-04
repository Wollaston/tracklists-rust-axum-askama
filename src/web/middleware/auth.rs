use crate::{Error, Result};
use axum::extract::FromRequestParts;
use axum::response::Response;
use axum::{async_trait, RequestPartsExt};
use axum::{body::Body, extract::Request, middleware::Next};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    request: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(request).await)
}

// region: --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> std::prelude::v1::Result<Self, Self::Rejection> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        // Cookies extractor
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let (user_id, _exp, _sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        // TODO: Token components validation.

        Ok(Ctx::new(user_id))
    }
}
// endregion: --- Ctx Extractor

/// Parse a token of format:
///     user-[user-id].[expiration].[signature]
/// Returns (user-id: u64, expiration: String, signature: String)
fn parse_token(token: String) -> Result<(uuid::Uuid, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"(^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})\.(.+)\.(.+)"#,
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: uuid::Uuid =
        uuid::Uuid::parse_str(user_id).map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}

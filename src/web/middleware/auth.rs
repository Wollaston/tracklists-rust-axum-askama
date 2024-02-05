use crate::model::ModelController;
use crate::{Error, Result};
use axum::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::response::Response;
use axum::{body::Body, extract::Request, middleware::Next};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    request: Request<Body>,
    next: Next,
) -> Result<Response> {
    info!("{:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(request).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    info!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>.
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO: Token components validations.
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);
    Ok(next.run(req).await)
}

// region: --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> std::prelude::v1::Result<Self, Self::Rejection> {
        info!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInReqExt)?
            .clone()
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

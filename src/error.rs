use askama_axum::{IntoResponse, Response};
use axum::http::StatusCode;
use serde::Serialize;
use tracing::info;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    DBError,

    //  --  Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInReqExt,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        info!("{:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // -- Auth
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInReqExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            // -- Model
            Self::DBError => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),
            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

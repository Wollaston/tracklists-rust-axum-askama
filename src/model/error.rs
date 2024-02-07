use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use super::store;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    //  --  Modules
    Store(store::error::Error),
    //  --  Externals
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl From<store::error::Error> for Error {
    fn from(value: store::error::Error) -> Self {
        Self::Store(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

use crate::{Error, Result};
use core::panic;
use std::{env, sync::OnceLock};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub TEMPLATES: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            TEMPLATES: get_env("SERVICE_TEMPLATES")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

use std::env;

use once_cell::sync::Lazy;
use serde::Deserialize;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::init().expect("Config init error"));

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub logger: Logger,
    pub database: Database,
}
#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}
impl Config {
    pub fn init() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::Environment::default().separator("_"));
        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }
        builder.build()?.try_deserialize()
    }
}

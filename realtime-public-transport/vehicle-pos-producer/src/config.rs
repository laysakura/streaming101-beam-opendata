use std::env;

use dotenvy::dotenv;

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct Config {
    pub(crate) kafka_servers: String,
    pub(crate) kafka_topic: String,
}

impl Config {
    pub(crate) fn from_env() -> Self {
        Self::load_dotenv();
        Self {
            kafka_servers: Self::get_var("KAFKA_SERVERS"),
            kafka_topic: Self::get_var("KAFKA_TOPIC"),
        }
    }

    fn load_dotenv() {
        match dotenv() {
            Ok(path) => log::info!("read environment variables from {}", path.to_string_lossy()),
            Err(_) => log::debug!(".env file not found"),
        }
    }

    fn get_var(env_name: &str) -> String {
        env::var(env_name)
            .unwrap_or_else(|_| panic!("environment variable `{}` is unset", env_name))
    }
}

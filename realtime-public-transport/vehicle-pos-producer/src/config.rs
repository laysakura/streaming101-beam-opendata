use std::{env, time::Duration};

use dotenvy::dotenv;

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct Config {
    pub(crate) kafka_servers: String,
    pub(crate) kafka_topic: String,

    pub(crate) vehicle_pos_api_key: String,
    pub(crate) vehicle_pos_api_url: String,
    pub(crate) vehicle_pos_api_mime: String,
    pub(crate) vehicle_pos_api_call_freq: Duration,
}

impl Config {
    pub(crate) fn from_env() -> Self {
        Self::load_dotenv();

        Self {
            kafka_servers: Self::get_var("KAFKA_SERVERS"),
            kafka_topic: Self::get_var("KAFKA_TOPIC"),
            vehicle_pos_api_key: Self::get_var("VEHICLE_POS_API_KEY"),
            vehicle_pos_api_url: Self::get_var("VEHICLE_POS_API_URL"),
            vehicle_pos_api_mime: Self::get_var("VEHICLE_POS_API_MIME"),
            vehicle_pos_api_call_freq: Self::env_to_duration("VEHICLE_POS_API_CALL_FREQ_SEC"),
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

    fn env_to_duration(env_name: &str) -> Duration {
        let s = Self::get_var(env_name);
        let u = s
            .parse::<u64>()
            .expect("failed to parse `VEHICLE_POS_API_CALL_FREQ_SEC` environment variable as u64");
        Duration::from_secs(u)
    }
}

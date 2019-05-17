//! Configuration data structure to be populated by the `envy` library.

use std::str::FromStr;
use log::{error, LevelFilter};

use serde::{Deserialize, Deserializer};


fn default_log_level() -> LevelFilter { LevelFilter::Debug }
fn deserialize_log_level<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    match LevelFilter::from_str(&s) {
        Ok(level) => Ok(level),
        Err(e) => {
            error!("Failed to parse log level:  {}", e);
            Ok(default_log_level())
        }
    }
}
fn default_cassandra_host() -> String { String::from("127.0.0.1") }
fn default_cassandra_port() -> i32 { 9042 }

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default="default_log_level", deserialize_with = "deserialize_log_level")]
    pub log_level: LevelFilter,
    #[serde(default="default_cassandra_host")]
    pub cassandra_host: String,
    #[serde(default="default_cassandra_port")]
    pub cassandra_port: i32,
}

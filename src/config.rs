#![allow(non_snake_case)]

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub WATCHDOG_RS_BUILD_KEY: String,
    pub WATCHDOG_RS_DISCORD_WEBHOOK: String,
    pub WATCHDOG_RS_REPOSITORY_URL: String,
    pub WATCHDOG_RS_BUILD_ID: String, // later add language and shit
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    services: HashMap<String, ServiceConfig>,
}

pub fn parse_watchdog_rs_yml(
    file_path: &str,
) -> Result<Vec<(String, ServiceConfig)>, Box<dyn Error>> {
    let file_content: String = fs::read_to_string(file_path)?;
    let config: Config = serde_yaml::from_str(&file_content)?;

    Ok(config.services.into_iter().collect())
}

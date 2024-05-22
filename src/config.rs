#![allow(dead_code)]

use anyhow::{Error, Result};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub game_name: String,
    pub tag_line: String,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
}

impl Config {
    pub fn build(
        file_path: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<Config, Error> {
        // read in config json and
        // construct headers from an external file or location (possibly provided by user)
        let text: String = fs::read_to_string(file_path)?.parse()?;
        let mut config: Config = serde_json::from_str(&text)?;

        // append start_time and end_time parameters from the user if provided
        config.start_time = start_time;
        config.end_time = end_time;
        Ok(config)
    }
}

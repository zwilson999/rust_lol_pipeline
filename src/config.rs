#![allow(dead_code)]

use anyhow::{Error, Result};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub game_name: String,
    pub tag_line: String,
}

impl Config {
    pub fn build(file_path: &str) -> Result<Config, Error> {
        // read in config json and
        // construct headers from an external file or location (possibly provided by user)
        let text: String = fs::read_to_string(file_path)?.parse()?;
        let config: Config = serde_json::from_str(&text)?;
        Ok(config)
    }
}

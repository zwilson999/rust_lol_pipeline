pub mod api;
pub mod config;
pub mod db;
pub mod pipeline;
pub mod utils;

use anyhow::{Error, Result};
use clap::Parser;
use config::{Config, UserArgs};
use pipeline::Pipeline;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let start = Instant::now();
    let args = UserArgs::parse();

    let config = Config::build(args);
    let account = config.get_api_key().await?;

    // run pipeline and check errors
    let pipeline = Pipeline::new(config, account);
    pipeline.run().await.unwrap_or_else(|err| {
        println!("ERROR: pipeline failed with err: {err}");
    });

    println!("INFO: Program finished in: {:.2?}", start.elapsed());
    Ok(())
}

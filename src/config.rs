#![allow(dead_code)]

use clap::Parser;
use sqlx::{Connection, PgConnection};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    user: String,
    #[arg(short, long)]
    pwd: String,
    #[arg(short, long)]
    summoner: String,
    #[arg(long)]
    start: String,
    #[arg(long)]
    end: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Account {
    pub game_name: String,
    pub api_key: String,
    pub tag_line: String,
}

pub async fn get_api_keys(user: &str, pwd: &str, summoner: &str) -> Result<Account, sqlx::Error> {
    let conn_string = format!(
        "postgresql://{user}:{pwd}@localhost/league_of_legends",
        user = user,
        pwd = pwd
    );
    let mut conn = PgConnection::connect(&conn_string).await?;

    let account_info = sqlx::query_as::<_, Account>(
        r#"
        SELECT game_name, api_key, tag_line 
        FROM league_of_legends.public.league_accounts
        WHERE game_name = $1;
        "#,
    )
    .bind(summoner)
    .fetch_one(&mut conn)
    .await?;

    Ok(account_info)
}

#[derive(Debug)]
pub struct Config {
    pub account: Account,
    pub start: u64,
    pub end: u64,
}

impl Config {
    pub fn build(account: Account, start: u64, end: u64) -> Self {
        Self {
            account,
            start,
            end,
        }
    }
}

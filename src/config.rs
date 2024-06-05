#![allow(dead_code)]
use clap::Parser;
use sqlx::{Connection, PgConnection};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
pub struct UserArgs {
    #[arg(short, long)]
    pub user: String,
    #[arg(long)]
    pub pwd: String,
    #[arg(long, default_value_t = String::from("Epoetin Alfa"))]
    pub summoner: String,
    #[arg(long, default_value_t = 1338253148)]
    pub start: u64,
    #[arg(long, default_value_t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())]
    pub end: u64,
    #[arg(short, long, default_value_t = 400)]
    // The queue type for matches, draft = 400, blind = 430, ARAM = 450
    pub queue_id: u16,
    #[arg(long, default_value_t = String::from("localhost"))]
    pub host: String,
    #[arg(short, long, default_value_t = 5432)]
    pub port: u16,
    #[arg(short, long, default_value_t = String::from("league_of_legends"))]
    pub db: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub user: String,
    pub pwd: String,
    pub pg_host: String,
    pub pg_port: u16,
    pub pg_db: String,
    pub summoner: String,
    pub queue_id: u16,
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct Account {
    pub game_name: String,
    pub api_key: String,
    pub tag_line: String,
}

impl Config {
    pub fn build(args: UserArgs) -> Self {
        Self {
            user: args.user,
            pwd: args.pwd,
            pg_host: args.host,
            pg_port: args.port,
            pg_db: args.db,
            summoner: args.summoner,
            queue_id: args.queue_id,
            start: args.start,
            end: args.end,
        }
    }

    pub async fn get_api_key(&self) -> Result<Account, sqlx::Error> {
        let conn_string = format!(
            "postgresql://{user}:{pwd}@localhost/league_of_legends",
            user = self.user,
            pwd = self.pwd
        );
        let mut conn = PgConnection::connect(&conn_string).await?;

        let account = sqlx::query_as::<_, Account>(
            r#"
            SELECT game_name, api_key, tag_line 
            FROM league_of_legends.public.league_accounts
            WHERE game_name = $1;
            "#,
        )
        .bind(&self.summoner)
        .fetch_one(&mut conn)
        .await?;

        Ok(account)
    }
}

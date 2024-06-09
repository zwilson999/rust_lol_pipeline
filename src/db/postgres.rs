use crate::api::lol_match_info::MatchResponse;
use crate::config::Config;
use anyhow::{Error, Result};

#[derive(Debug, Clone)]
pub struct Postgres {
    pub conn_string: String,
    pub load_id: String,
    // pub raw_tbl: String,
    // pub flat_tbl: String,
}

impl Postgres {
    pub fn new(config: &Config, load_id: String) -> Self {
        // create conn string
        Self {
            conn_string: format!(
                "postgresql://{user}:{pwd}@localhost/{db}",
                user = config.user,
                pwd = config.pwd,
                db = config.pg_db,
            ),
            load_id,
            // raw_tbl,
            // flat_tbl,
        }
    }

    pub async fn get_pool(&self) -> Result<sqlx::PgPool, Error> {
        let pool = sqlx::PgPool::connect(&self.conn_string).await?;
        Ok(pool)
    }

    pub async fn load(self, response: &MatchResponse, pool: sqlx::PgPool) -> Result<(), Error> {
        let _ = sqlx::query(
            r#"
            INSERT INTO
                league_of_legends.raw.league_matches
            VALUES(
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            )
            "#,
        )
        .bind(&response.account_name)
        .bind(&response.info.queue_id)
        .bind(&response.info.game_type)
        .bind(&response.metadata.match_id)
        .bind(serde_json::to_string(&response).unwrap())
        .bind(self.load_id.to_string())
        .execute(&pool)
        .await?;

        Ok(())
    }

    // pub async fn flatten(self, pool: sqlx::PgPool) -> Result<(), Error> {
    //     Ok(())
    // }
}

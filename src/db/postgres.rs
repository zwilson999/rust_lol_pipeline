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
        }
    }

    pub async fn get_pool(&self) -> Result<sqlx::PgPool, Error> {
        let pool = sqlx::PgPool::connect(&self.conn_string).await?;
        Ok(pool)
    }

    pub async fn load(
        self,
        response: &MatchResponse,
        pool: sqlx::PgPool,
    ) -> Result<(), sqlx::Error> {
        // create a db transaction
        let mut tx = pool.begin().await?;

        // insert data into temp table so we can use it as source to merge into raw
        let res = sqlx::query(&format!(
            r#"
            CREATE TEMP TABLE {match_id}_tmp (account_name, queue_id_param, queue_type, match_id, response_json, load_id)
            AS
            VALUES(
                $1, $2, $3, $4, $5, $6
            );
            "#,
            match_id = response.match_id
        ))
            .bind(&response.account_name)
            .bind(&response.info.queue_id)
            .bind(&response.info.game_type)
            .bind(&response.metadata.match_id)
            .bind(sqlx::types::Json(&response)) // cast to JSON type for postgres compatibility
            .bind(self.load_id.to_string())
            .execute(&mut *tx)
            .await?;
        println!("INFO: insert result {:?}", res);

        // query to merge data into raw using the match_id

        // query to merge into raw target
        let res = sqlx::query(&format!(
            r#" 
            MERGE INTO
                league_of_legends.raw.league_matches AS tgt
            USING
                {match_id}_tmp AS src
            ON
                src.match_id = tgt.match_id
            WHEN NOT MATCHED THEN
                INSERT (
                    account_name, queue_id_param, queue_type, match_id, response_json, load_id
                )
                VALUES (
                    src.account_name, src.queue_id_param, src.queue_type, src.match_id, src.response_json, src.load_id
                );
            "#,
            match_id = response.match_id
        ))
        .execute(&mut *tx)
        .await?;
        println!("INFO: merge result {:?}", res);

        // commit the transaction
        tx.commit().await?;

        // check for errors during our transaction
        // tx.rollback().await?;
        println!(
            "INFO: successfully loaded {match_id} to postgres",
            match_id = response.match_id
        );
        Ok(())
    }

    // pub async fn flatten(self, pool: sqlx::PgPool) -> Result<(), Error> {
    //     Ok(())
    // }
}

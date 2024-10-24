#![allow(dead_code)]
use crate::api::lol_match_data::MatchRequest;
use crate::api::lol_matches::MatchesRequest;
use crate::api::lol_riot_account::{AccountInfoRequest, RiotAccount};
use crate::config::{Account, Config};
use crate::db::postgres::Postgres;

use anyhow::{Error, Result};
use core::pin::pin;
use futures::stream::{self as stream, StreamExt};
use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_CHARSET, ACCEPT_LANGUAGE};
use tokio::time::{sleep, Duration, Instant};
use uuid::Uuid;

pub struct Pipeline /*<'a>*/ {
    pub config: Config,
    pub account: Account,
    pub load_id: String,
    pub headers: HeaderMap,
    // pub global_token_bucket: Arc<TokenBucket>,
    // pub local_token_bucket: Arc<TokenBucket>,
}

static LOCAL_REQUEST_LIMIT: usize = 20; // 20 requests
static LOCAL_REQUEST_INTERVAL: f32 = 1.0; // 1 second

impl Pipeline {
    pub fn new(config: Config, account: Account) -> Self {
        // create headers for all API call
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(
            ACCEPT_CHARSET,
            "application/x-www-form-urlencoded; charset=UTF-8"
                .parse()
                .expect("ERROR: could not parse headers properly"),
        );
        headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.5".parse().unwrap());
        headers.insert("X-Riot-Token", account.api_key.parse().unwrap());

        // let local_update_interval =
        //     Duration::from_secs_f32(LOCAL_REQUEST_INTERVAL / LOCAL_REQUEST_LIMIT as f32);

        Self {
            config,
            account,
            load_id: Uuid::new_v4().to_string(),
            headers,
        }
    }

    pub async fn run(self) -> Result<(), Error> {
        // get information for the account we received api key for
        let account_info = self.get_account_info().await?;

        // get all matches for the account
        let _ = self.load_matches(account_info.puuid).await;

        Ok(())
    }

    // make a request to get account information from API
    async fn get_account_info(&self) -> Result<RiotAccount, Error> {
        let start = Instant::now();

        // make reqest
        sleep(Duration::from_secs_f32(1.0)).await;
        let req = AccountInfoRequest::new(
            &self.account.game_name,
            &self.account.tag_line,
            self.headers.clone(),
        );

        // let permit = self.sem.acquire().await.unwrap();
        let resp = match req.get().await {
            Ok(resp) => resp,
            Err(err) => {
                panic!("ERROR: could not receive PUUID, error: {err}");
            }
        };
        println!(
            "INFO: Riot Account Info Request took: {:.2?}",
            start.elapsed()
        );

        // parse the account data into a struct
        let data = resp.json::<RiotAccount>().await?;
        // drop(permit);
        Ok(data)
    }

    async fn load_matches(&self, puuid: String) -> Result<(), Error> {
        // run stream to process match ids and load into database
        let client = reqwest::Client::new();
        let mut s = pin!(stream::iter((0..).step_by(100))
            .filter_map(|idx| {
                let client = client.clone();
                let puuid = puuid.clone();
                let config = self.config.clone();
                async move {
                    // sleep async 1s before making request
                    sleep(Duration::from_secs_f32(1.0)).await;
                    let req = MatchesRequest::new(config, puuid, Some(idx));
                    let resp = req.get(client, self.headers.clone()).await.ok()?;
                    Some(resp)
                }
            })
            .take_while(|r| {
                // continue streaming until we have a response with 0 records
                futures::future::ready(r.len() > 0)
            })
            // make our Vec<String> responses into a single stream
            .flat_map(stream::iter)
            .flat_map_unordered(20, |m| {
                let match_id = m.clone();
                let client = client.clone();
                Box::pin(stream::once(async move {
                    // sleep async 1s before making request
                    sleep(Duration::from_secs_f32(1.0)).await;
                    let match_request = MatchRequest::new(match_id.to_string());
                    let mut resp = match_request.get(client, self.headers.clone()).await.ok()?;

                    // add some useful metadata fields
                    resp.account_name = self.account.game_name.clone();
                    Some(resp)
                }))
            }));

        let mut i = 0;
        let pg = Postgres::new(&self.config, self.load_id.to_string());
        let pg_pool = pg.get_pool().await?;
        while let Some(m) = s.next().await {
            let pg_pool = pg_pool.clone();
            let pg = pg.clone();

            // if the item is Some, load it, otherwise skip it
            if let Some(m) = m {
                let _ = pg.load(&m, pg_pool).await.unwrap_or_else(|err| {
                    println!("ERROR: could not load match to postgres. err: {err}",);
                });
                i += 1;
                println!("INFO: processed {i}th LOL match");
            }
        }
        println!("INFO: ingested {} matches total", i);
        Ok(())
    }
}

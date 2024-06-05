#![allow(dead_code)]
use crate::api::lol_match_data::MatchRequest;
use crate::api::lol_match_info::MatchResponse;
use crate::api::lol_matches::MatchesRequest;
use crate::api::lol_riot_account::{AccountInfoRequest, RiotAccount};
use crate::config::{Account, Config};
use crate::utils::bucket::TokenBucket;

use anyhow::{Error, Result};
use core::pin::pin;
use futures::stream::{self as stream, StreamExt};
use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_CHARSET, ACCEPT_LANGUAGE};
use std::sync::Arc;
use std::time::Instant;
use tokio::time::Duration;
use uuid::Uuid;

pub struct Pipeline /*<'a>*/ {
    pub config: Config,
    pub account: Account,
    pub load_id: Uuid,
    pub headers: HeaderMap,
    pub global_token_bucket: Arc<TokenBucket>,
    pub local_token_bucket: Arc<TokenBucket>,
}

// global request limits are 100 requests per 120 seconds
static GLOBAL_REQUEST_LIMIT: usize = 100; // 100 requests
static GLOBAL_REQUEST_INTERVAL: f32 = 120.0; // 2 minutes

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

        // construct pipeline global rate limit token bucket
        let global_update_interval =
            Duration::from_secs_f32(GLOBAL_REQUEST_INTERVAL / GLOBAL_REQUEST_LIMIT as f32);

        let local_update_interval =
            Duration::from_secs_f32(LOCAL_REQUEST_INTERVAL / LOCAL_REQUEST_LIMIT as f32);

        Self {
            config,
            account,
            load_id: Uuid::new_v4(),
            headers,
            global_token_bucket: Arc::new(TokenBucket::new(
                global_update_interval,
                GLOBAL_REQUEST_LIMIT,
            )),
            local_token_bucket: Arc::new(TokenBucket::new(
                local_update_interval,
                LOCAL_REQUEST_LIMIT,
            )),
        }
    }

    pub async fn run(self) -> Result<(), Error> {
        // get information for the account we received api key for
        let account_info = self.get_account_info().await?;

        // get all matches for the account
        let _ = self.get_matches(account_info.puuid).await;

        Ok(())
    }

    // make a request to get account information from API
    async fn get_account_info(&self) -> Result<RiotAccount, Error> {
        // acquire global semaphore
        self.global_token_bucket.acquire().await;

        // make reqest
        let req = AccountInfoRequest::new(
            &self.account.game_name,
            &self.account.tag_line,
            self.headers.clone(),
        );

        let start = Instant::now();
        self.local_token_bucket.acquire().await;
        self.global_token_bucket.acquire().await;
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
        Ok(data)
    }

    async fn get_matches(&self, puuid: String) -> Result<(), Error> {
        // create rate limiter for 20 calls per second
        let client = reqwest::Client::new();
        let mut s = pin!(stream::iter((0..).step_by(100))
            .filter_map(|idx| {
                let client = client.clone();
                let puuid = puuid.clone();
                let config = self.config.clone();
                let local_bucket = self.local_token_bucket.clone();
                let global_bucket = self.global_token_bucket.clone();
                async move {
                    local_bucket.acquire().await;
                    global_bucket.acquire().await;
                    let req = MatchesRequest::new(config, puuid, Some(idx));
                    let resp = (req.get(client, self.headers.clone()).await).ok()?;
                    Some(resp)
                }
            })
            .take_while(|r| {
                // continue streaming until we have a response with non-full amount of match ids
                futures::future::ready(r.len() == 0)
            })
            // make our Vec<String> responses into a single stream
            .flat_map(stream::iter)
            .flat_map_unordered(20, |m| {
                let match_id = m.clone();
                let client = client.clone();
                let local_bucket = self.local_token_bucket.clone();
                let global_bucket = self.global_token_bucket.clone();
                Box::pin(stream::once(async move {
                    local_bucket.acquire().await;
                    global_bucket.acquire().await;
                    let match_request = MatchRequest::new(match_id.to_string());
                    let resp = (match_request.get(client, self.headers.clone()).await).ok()?;
                    Some(resp)
                }))
            }));

        let mut i = 0;
        while let Some(m) = s.next().await {
            // println!("match info: {:?}", m);
            i += 1;
        }
        println!("ingested {} matches", i);
        Ok(())

        /*
        // attempt 2
        let (matches_tx, mut matches_rx) = mpsc::channel::<Vec<String>>(5);
        let (match_tx, mut match_rx) = mpsc::channel::<MatchResponse>(5);
        let client = reqwest::Client::new();

        // create iterator that will stream async responses
        tokio::spawn(async move {
            let _ = futures::stream::iter((0..).step_by(100))
                .then(|idx| {
                    let client = client.clone();
                    let req = MatchesRequest::new(self.config.clone(), puuid.clone(), Some(idx));
                    req.get(client, self.headers.clone())
                })
                // .and_then(|resp| resp.json::<Vec<String>>())
                .try_for_each_concurrent(2, |r| async {
                    let matches_tx_cloned = matches_tx.clone();
                    let _ = matches_tx_cloned.send(r).await;
                    Ok(())
                })
                .await;
        });

        while let Some(matches) = matches_rx.recv().await {
            println!("received {} matches", matches.len());

            // iterate over vec of posts to spawn other concurrent requests
            let match_vec = matches.clone();
            let new_client = reqwest::Client::new();
            let match_tx_cloned = match_tx.clone();
            tokio::spawn(async move {
                for m in match_vec {
                    let cloned_client = new_client.clone();
                    let match_request = MatchRequest::new(m.to_string());
                    let resp = match_request.get(cloned_client, self.headers.clone()).await;
                    match_tx_cloned.send(resp.unwrap()).await.unwrap();
                }
            });

            // quit if the response comes back empty
            if matches.len() == 0 {
                break;
            }
        }

        // read from the user channel
        while let Some(match_info) = match_rx.recv().await {
            println!("found match {:?}", match_info);
        }

        Ok(())
        */
        /*
        // create rate limiter for 20 calls per second
        let rl = RateLimiter::direct(Quota::per_second(std::num::NonZeroU32::new(20u32).unwrap()));
        let mut s = pin!(stream::iter((0..).step_by(100))
            // .ratelimit_stream(&rl)
            .filter_map(|idx| {
                let client = client.clone();
                let puuid = puuid.clone();
                let config = self.config.clone();
                Box::pin(async move {
                    let req = MatchesRequest::new(config, puuid, Some(idx));
                    let resp = (req.get(client, self.headers.clone()).await).ok()?;
                    println!("{:?}", resp);
                    Some(resp)
                })
            })
            .take_while(|r| {
                // continue streaming until we have a response with non-full amount of match ids
                futures::future::ready(r.len() == 100)
            })
            // make our Vec<String> responses into a single stream
            .flat_map(stream::iter)
            .flat_map_unordered(20, |m| {
                let match_id = m.clone();
                let client = client.clone();
                Box::pin(stream::once(async move {
                    let match_request = MatchRequest::new(match_id.to_string());
                    let resp = (match_request.get(client, self.headers.clone()).await).ok()?;
                    Some(resp)
                }))
            }));

        let mut i = 0;
        while let Some(m) = s.next().await {
            // println!("{:?}", m);
            i += 1;
        }
        println!("ingested {} matches", i);
        */

        /*
        tokio::spawn(async move {
            let _ = stream::iter((0..).step_by(100))
                .then(|idx| {
                    let client = &client;
                    let req = MatchesRequest::new(&self.config, &self.account, &puuid, Some(idx));
                    req.get(client, self.headers.clone())
                })
                .and_then(|resp| resp.json())
                .try_for_each_concurrent(2, |r| async {
                    // acquire semaphore, note it will automatically be dropped
                    let tx_cloned = tx.clone();
                    let _ = tx_cloned.send(r).await;
                    Ok(())
                })
                .await;
        });

        // read from channel and write to database as data is received
        // note we will block on this until we are finished
        // additionally, we will use a token bucket to limit our concurrency to honor
        // the 20 requests per second rate limit
        while let Some(resp) = rx.recv().await {
            // println!("{:?}", resp);

            // acquire both semaphores as they correspond to different rate limits
            self.global_token_bucket.acquire().await;
            self.local_token_bucket.acquire().await;

            // make match requests for every match within our response vector
            // let matches = Arc::new(resp.clone());
            // let handles = matches.iter().map(|match_id| {
            //     tokio::spawn(async move {
            //         let client = &client;
            //         let match_request = MatchRequest::new(match_id.to_string());
            //         match_request.get(client, self.headers);
            //     })
            // });

            // if we received a page with < 100 items, we've reached the last page
            if resp.len() < 100 {
                break;
            }
        }
        */
    }
    // attempt 1
    // async fn get_matches(&self, puuid: String) -> Result<(), Error> {
    //     // receive all matches of the below types for the given puuid
    //     let matches = MatchesRequest::new(&self.config, &self.account, &puuid);
    //
    //     // create channel which will consume matches as they arrive
    //     let (tx, mut rx) = mpsc::channel(100);
    //     for match_id in matches {
    //         match match_id {
    //             Ok(m) => {
    //                 let tx_cloned = tx.clone();
    //                 let api_key = self.account.api_key.to_owned();
    //                 let req = MatchRequest::new(api_key, m, tx_cloned);
    //
    //                 // make an async match request
    //                 tokio::spawn(async move {
    //                     let _ = req.get_async().await;
    //                 });
    //             }
    //             Err(e) => {
    //                 println!("ERROR: could not receive match id from matches request: {e}");
    //             }
    //         };
    //     }
    //
    //     // read from channel and write to database as data is received
    //     // note we will block on this until we are finished
    //     while let Some(resp) = rx.recv().await {
    //         println!("{:?}", resp);
    //     }
    //
    //     Ok(())
    // }
}

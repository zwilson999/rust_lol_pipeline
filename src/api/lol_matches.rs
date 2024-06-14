use crate::config::Config;
use anyhow::{Error, Result};
use reqwest::header::HeaderMap;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};

#[derive(Serialize, Debug)]
pub struct MatchesQuery<'b> {
    #[serde(rename(serialize = "startTime"))]
    pub start_time: u64,
    #[serde(rename(serialize = "endTime"))]
    pub end_time: u64,
    #[serde(rename(serialize = "queue"))]
    pub queue_id: u16,
    #[serde(rename(serialize = "type"))]
    pub r#type: &'b str,
    #[serde(rename(serialize = "start"))]
    pub start_idx: u16,
    #[serde(rename(serialize = "count"))]
    pub page_size: u16,
}

impl<'b> Default for MatchesQuery<'b> {
    fn default() -> Self {
        Self {
            start_time: 1338253148,
            end_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|err| {
                    panic!("ERROR: could not get SYSTEM_TIME since UNIX_EPOCH: {err}");
                })
                .as_secs(),
            queue_id: 400, // default to draft
            r#type: "normal",
            start_idx: 0,
            page_size: 100,
        }
    }
}

#[allow(dead_code)]
impl<'b> MatchesQuery<'b> {
    pub fn new(
        start_time: u64,
        end_time: u64,
        queue_id: u16,
        r#type: &'b str,
        start_idx: u16,
        page_size: u16,
    ) -> Self {
        // fill the struct with non-default options
        Self {
            start_time,
            end_time,
            queue_id,
            r#type,
            start_idx,
            page_size,
        }
    }
}

#[derive(Debug)]
pub struct MatchesRequest<'b> {
    pub puuid: String,
    pub query: MatchesQuery<'b>,
}

impl<'b> Default for MatchesRequest<'b> {
    fn default() -> Self {
        Self {
            puuid: String::from(""),
            query: MatchesQuery::default(),
        }
    }
}

#[allow(dead_code)]
impl<'b> MatchesRequest<'b> {
    pub fn new(config: Config, puuid: String, idx: Option<u16>) -> Self {
        Self {
            puuid,
            query: MatchesQuery::new(
                config.start.clone(),
                config.end.clone(),
                config.queue_id.clone(),
                "normal",
                idx.unwrap_or_default(),
                100,
            ),
        }
    }

    pub async fn get(
        self,
        client: reqwest::Client,
        headers: HeaderMap,
    ) -> Result<Vec<String>, Error> {
        // format url
        let url = format!(
            "https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/{puuid}/ids",
            puuid = self.puuid
        );

        loop {
            let resp = client
                .get(&url)
                .headers(headers.clone())
                .query(&self.query)
                .send()
                .await?;

            // check status and return appropriate response
            match resp.status() {
                reqwest::StatusCode::OK => {
                    let data = resp.json::<Vec<String>>().await?;
                    return Ok(data);
                }
                reqwest::StatusCode::BAD_REQUEST => {
                    let err = format!("ERROR: bad request. status code: {}", resp.status());
                    panic!("{}", err);
                }
                reqwest::StatusCode::FORBIDDEN => {
                    let err = format!(
                        "ERROR: forbidden request. check credentials. status code: {}",
                        resp.status()
                    );
                    panic!("{}", err);
                }
                reqwest::StatusCode::TOO_MANY_REQUESTS => {
                    // if for some reason the header cannot be found, default it to 2 minutes (max sleep for rate limits to recover)
                    let retry_after: &str = resp
                        .headers()
                        .get(reqwest::header::RETRY_AFTER)
                        .and_then(|s| s.to_str().ok())
                        .unwrap_or("120");

                    println!(
                        "INFO: rate limit reached when querying matches. sleeping for {retry_after}s",
                        retry_after = &retry_after
                    );
                    // sleep for retry_after seconds
                    sleep(Duration::from_secs(
                        retry_after.parse::<u64>().unwrap_or(120),
                    ))
                    .await;
                    continue;
                }
                _ => {
                    let err = format!("unsavory request. status code: {}", resp.status());
                    eprintln!("{}", err);
                    return Err(anyhow::anyhow!(err));
                }
            };
        }
    }
}

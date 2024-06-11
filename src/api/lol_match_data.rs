use crate::api::lol_match_info::MatchResponse;
use anyhow::{Error, Result};
use reqwest::header::HeaderMap;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub struct MatchRequest {
    match_id: String,
}

#[allow(dead_code)]
impl MatchRequest {
    pub fn new(match_id: String) -> Self {
        Self { match_id }
    }

    pub async fn get(
        self,
        client: reqwest::Client,
        headers: HeaderMap,
    ) -> Result<MatchResponse, Error> {
        let url = format!(
            "https://americas.api.riotgames.com/lol/match/v5/matches/{match_id}",
            match_id = self.match_id
        );
        loop {
            let resp = client.get(&url).headers(headers.clone()).send().await?;

            // check status and return appropriate response
            match resp.status() {
                reqwest::StatusCode::OK => {
                    let mut data = resp.json::<MatchResponse>().await.unwrap_or_else(|err| {
                        panic!(
                            "ERROR: match {} had errors with deserializing to JSON, err: {err}",
                            &self.match_id
                        );
                    });
                    data.match_id = self.match_id.clone();
                    return Ok(data);
                }
                reqwest::StatusCode::BAD_REQUEST => {
                    panic!(
                        "ERROR: match {} received BAD_REQUEST status.",
                        &self.match_id
                    );
                }
                reqwest::StatusCode::FORBIDDEN => {
                    panic!(
                        "ERROR: forbidden request for match {}. check credentials and try again.",
                        &self.match_id,
                    );
                }
                reqwest::StatusCode::TOO_MANY_REQUESTS => {
                    // if for some reason the header cannot be found, default it to 2 minutes (max sleep for rate limits to recover)
                    let retry_after: &str = resp
                        .headers()
                        .get(reqwest::header::RETRY_AFTER)
                        .and_then(|s| s.to_str().ok())
                        .unwrap_or("120");

                    println!(
                        "INFO: rate limit reached when querying a single match. sleeping for {retry_after}s",
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
                    let err = format!("ERROR: unsavory request. status code: {}", resp.status());
                    println!("{}", err);
                    return Err(anyhow::anyhow!(err));
                }
            }
        }
    }
}

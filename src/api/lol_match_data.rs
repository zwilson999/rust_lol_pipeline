use crate::api::lol_match_info::MatchResponse;
use anyhow::{Error, Result};
use reqwest::header::HeaderMap;

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
        let resp = client.get(url).headers(headers).send().await?;

        // check status and return appropriate response
        match resp.status() {
            reqwest::StatusCode::OK => {
                let data = resp.json::<MatchResponse>().await.unwrap_or_else(|err| {
                    println!("match: {} had errors", &self.match_id);
                    panic!("{err}");
                });
                return Ok(data);
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let err = format!("bad status. status code: {}", resp.status());
                return Err(anyhow::anyhow!(err));
            }
            reqwest::StatusCode::FORBIDDEN => {
                let err = format!(
                    "forbidden request. check credentials. status code: {}",
                    resp.status()
                );
                return Err(anyhow::anyhow!(err));
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                let err = format!("too many requests. status code: {}", resp.status());
                return Err(anyhow::anyhow!(err));
            }
            _ => {
                let err = format!("unsavory request. status code: {}", resp.status());
                return Err(anyhow::anyhow!(err));
            }
        };
    }
}

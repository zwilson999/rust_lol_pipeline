use anyhow::{Error, Result};
use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_CHARSET, ACCEPT_LANGUAGE};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct MatchRequest {
    api_key: String,
    match_id: String,
    sender: mpsc::Sender<reqwest::Response>,
}

#[allow(dead_code)]
impl MatchRequest {
    pub fn new(api_key: String, match_id: String, tx: mpsc::Sender<reqwest::Response>) -> Self {
        Self {
            api_key,
            match_id,
            sender: tx,
        }
    }

    fn create_headers(&self) -> HeaderMap {
        // headers for the API call
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(
            ACCEPT_CHARSET,
            "application/x-www-form-urlencoded; charset=UTF-8"
                .parse()
                .unwrap(),
        );
        headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.5".parse().unwrap());
        headers.insert("X-Riot-Token", self.api_key.parse().unwrap());
        headers
    }

    pub async fn get_async(&self) -> Result<reqwest::Response, Error> {
        // create async http client
        let client = reqwest::Client::new();
        let url = format!(
            "https://americas.api.riotgames.com/lol/match/v5/matches/{match_id}",
            match_id = self.match_id
        );
        let resp = client
            .get(url)
            .headers(self.create_headers())
            .send()
            .await?;

        // check status and return appropriate response
        match resp.status() {
            reqwest::StatusCode::OK => {
                return Ok(resp);
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
            _ => {
                let err = format!("unsavory request. status code: {}", resp.status());
                return Err(anyhow::anyhow!(err));
            }
        };
    }
}

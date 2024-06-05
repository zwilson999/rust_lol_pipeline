use anyhow::{Error, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RiotAccount {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
}

#[derive(Debug)]
pub struct AccountInfoRequest<'b> {
    pub game_name: &'b str,
    pub tag_line: &'b str,
    pub headers: reqwest::header::HeaderMap,
}

#[allow(dead_code)]
impl<'b> AccountInfoRequest<'b> {
    pub fn new(game_name: &'b str, tag_line: &'b str, headers: reqwest::header::HeaderMap) -> Self {
        Self {
            game_name,
            tag_line,
            headers,
        }
    }

    pub async fn get(&self) -> Result<reqwest::Response, Error> {
        // url to get the riot account info with access token
        let url = format!(
            "https://americas.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
            self.game_name, self.tag_line
        );

        let client = reqwest::Client::new();
        let resp = client.get(url).headers(self.headers.clone()).send().await?;

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

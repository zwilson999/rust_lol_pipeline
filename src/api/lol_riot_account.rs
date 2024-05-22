use anyhow::{Error, Result};
use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_CHARSET, ACCEPT_LANGUAGE};
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
    pub api_key: &'b str,
    pub game_name: &'b str,
    pub tag_line: &'b str,
}

#[allow(dead_code)]
impl<'b> AccountInfoRequest<'b> {
    pub fn new(api_key: &'b str, game_name: &'b str, tag_line: &'b str) -> Self {
        Self {
            api_key,
            game_name,
            tag_line,
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

    pub fn get_blocking(&self) -> Result<reqwest::blocking::Response, Error> {
        // url to get the riot account info with access token
        let url = format!(
            "https://americas.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
            self.game_name, self.tag_line
        );

        let client = reqwest::blocking::Client::new();
        let resp = client.get(url).headers(self.create_headers()).send()?;

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

use anyhow::{Error, Result};
use reqwest::header::HeaderMap;
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
    pub url: &'b str,
    pub headers: HeaderMap,
    pub game_name: &'b str,
    pub tag_line: &'b str,
    pub puuid: &'b str,
}

impl<'b> Default for AccountInfoRequest<'b> {
    fn default() -> Self {
        Self {
            api_key: "",
            url: "",
            headers: HeaderMap::new(),
            game_name: "",
            tag_line: "NA1",
            puuid: "",
        }
    }
}

#[allow(dead_code)]
impl<'b> AccountInfoRequest<'b> {
    pub fn new(
        api_key: &'b str,
        url: &'b str,
        headers: Option<HeaderMap>,
        game_name: Option<&'b str>,
        tagline: Option<&'b str>,
        puuid: Option<&'b str>,
    ) -> Self {
        Self {
            api_key,
            url,
            headers: headers.unwrap_or_default(),
            game_name: game_name.unwrap_or_default(),
            tag_line: tagline.unwrap_or_default(),
            puuid: puuid.unwrap_or_default(),
        }
    }

    pub fn get_blocking(&self) -> Result<reqwest::blocking::Response, Error> {
        let client = reqwest::blocking::Client::new();
        let resp = client.get(self.url).headers(self.headers.clone()).send()?;

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

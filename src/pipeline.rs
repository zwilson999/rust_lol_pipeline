#![allow(dead_code)]
use crate::api::lol_matches::MatchesRequest;
use crate::api::lol_riot_account::RiotAccount;
use crate::api::request::AccountInfoRequest;
use crate::config::Config;
use anyhow::{Error, Result};
use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_CHARSET, ACCEPT_LANGUAGE};
use std::time::Instant;
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct Pipeline /*<'a>*/ {
    pub config: Config,
    pub load_id: Uuid,
}

impl Pipeline {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            load_id: Uuid::new_v4(),
        }
    }

    pub fn run(self) -> Result<(), Error> {
        // start by getting our summoner information so we can use the data to make
        // further api calls
        let account_info = self
            .get_account_info()
            .unwrap_or_else(|err| { eprintln!(
                "ERROR: could not get account info for riot game name {} and tagline {}, exiting due to {} ",
                self.config.game_name, self.config.tag_line, err
            );
            std::process::exit(1);
        });

        // get all matches for the account
        self.get_matches(account_info.puuid)?;
        Ok(())
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
        headers.insert("X-Riot-Token", self.config.api_key.parse().unwrap());
        headers
    }

    fn get_account_info(&self) -> Result<RiotAccount, Error> {
        // url to get the riot account info with access token
        let url = format!(
            "https://americas.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
            self.config.game_name, self.config.tag_line
        );

        let req = AccountInfoRequest {
            api_key: &self.config.api_key,
            url: url.as_str(),
            headers: self.create_headers(),
            ..Default::default()
        };

        // make a request to get account information
        let start = Instant::now();
        let Ok(resp) = req.get_blocking() else {
            // end the program since we need the PUUID for the rest of the pipeline
            eprintln!("ERROR: could not receive PUUID");
            std::process::exit(1);
        };
        println!(
            "INFO: Riot Account Info Request took: {:.2?}",
            start.elapsed()
        );

        // parse the account data into a particular struct
        let data = resp.json::<RiotAccount>()?;
        Ok(data)
    }

    fn get_matches(&self, puuid: String) -> Result<(), Error> {
        // receive all matches of the below types for the given puuid
        let matches = MatchesRequest::new(
            self.create_headers(),
            &puuid,
            self.config.start_time,
            self.config.end_time,
            400,
            "normal",
            0,
            100,
        );

        // create channel which will consume matches as they arrive
        let (tx, mut rx) = mpsc::channel(100);
        for m in matches {}

        Ok(())
    }
}

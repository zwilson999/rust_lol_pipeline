#![allow(dead_code)]
use crate::api::lol_match_data::MatchRequest;
use crate::api::lol_match_info::MatchResponse;
use crate::api::lol_matches::MatchesRequest;
use crate::api::lol_riot_account::{AccountInfoRequest, RiotAccount};
use crate::config::Config;
use anyhow::{Error, Result};
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
        self.get_matches(account_info.puuid);
        Ok(())
    }

    fn get_account_info(&self) -> Result<RiotAccount, Error> {
        let req = AccountInfoRequest::new(
            &self.config.api_key,
            &self.config.game_name,
            &self.config.tag_line,
        );

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

    async fn get_matches(&self, puuid: String) -> Result<(), Error> {
        // receive all matches of the below types for the given puuid
        let matches = MatchesRequest::new(
            &self.config.api_key,
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
        for match_id in matches {
            match match_id {
                Ok(m) => {
                    let tx_cloned = tx.clone();
                    let api_key = self.config.api_key.to_owned();
                    let req = MatchRequest::new(api_key, m, tx_cloned);

                    // make an async  match request
                    tokio::spawn(async move {
                        if let Ok(resp) = req.get_async().await {
                            println!("{:?}", resp)
                        }
                    });
                }
                Err(e) => {
                    println!("ERROR: could not receive match id from matches request: {e}");
                }
            };
        }

        // read from channel and write to database as data is received
        while let Some(resp) = rx.recv().await {}

        Ok(())
    }
}

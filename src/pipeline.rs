#![allow(dead_code)]
use crate::api::lol_match_data::MatchRequest;
use crate::api::lol_match_info::MatchResponse;
use crate::api::lol_matches::MatchesRequest;
use crate::api::lol_riot_account::{AccountInfoRequest, RiotAccount};
use crate::config::{Account, Config};
use anyhow::{Error, Result};
use std::time::Instant;
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct Pipeline /*<'a>*/ {
    pub config: Config,
    pub account: Account,
    pub load_id: Uuid,
}

impl Pipeline {
    pub fn new(config: Config, account: Account) -> Self {
        Self {
            config,
            account,
            load_id: Uuid::new_v4(),
        }
    }

    pub async fn run(self) -> Result<(), Error> {
        // create a runtime since we need some elements of the pipeline to be synchronous
        let rt = tokio::runtime::Runtime::new().unwrap();

        // get information for the account we received api key for
        let account_info = rt.block_on(async {
            self.get_account_info()
                .unwrap_or_else(|err| { eprintln!(
                    "ERROR: could not get account info for riot game name {} and tagline {}, exiting due to {} ",
                    self.config.account.game_name, self.config.account.tag_line, err
                );
                std::process::exit(1);
            })
        });

        println!("{:?}", account_info);

        // get all matches for the account
        // let _ = self.get_matches(account_info.puuid).await;

        Ok(())
    }

    fn get_account_info(&self) -> Result<RiotAccount, Error> {
        let req = AccountInfoRequest::new(
            &self.config.account.api_key,
            &self.config.account.game_name,
            &self.config.account.tag_line,
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
            &self.config.account.api_key,
            &puuid,
            self.config.start,
            self.config.end,
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
                    let api_key = self.config.account.api_key.to_owned();
                    let req = MatchRequest::new(api_key, m, tx_cloned);

                    // make an async  match request
                    tokio::spawn(async move {
                        let _ = req.get_async().await;
                    });
                }
                Err(e) => {
                    println!("ERROR: could not receive match id from matches request: {e}");
                }
            };
        }

        // create async runtime
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        // read from channel and write to database as data is received
        // note we will block on this until we are finished
        rt.block_on(async move {
            while let Some(resp) = rx.recv().await {
                println!("{:?}", resp);
            }
        });

        Ok(())
    }
}

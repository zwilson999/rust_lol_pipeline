use anyhow::{Error, Result};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Debug)]
pub struct MatchesResponse {
    pub matches: Vec<Match>,
}

#[derive(Deserialize, Debug)]
pub struct Match {
    pub match_id: String,
}

#[derive(Serialize, Debug)]
pub struct Query<'b> {
    #[serde(rename(serialize = "startTime"))]
    pub start_time: Option<u64>,
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

impl<'b> Default for Query<'b> {
    fn default() -> Self {
        Self {
            start_time: None,
            end_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|err| {
                    println!("ERROR: could not get SYSTEM_TIME since UNIX_EPOCH: {err}");
                    std::process::exit(1);
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
impl<'b> Query<'b> {
    pub fn new(
        start_time: Option<u64>,
        end_time: Option<u64>,
        queue_id: u16,
        r#type: &'b str,
        start_idx: u16,
        page_size: u16,
    ) -> Self {
        let mut qry = Query::default();

        // check for start and end time values
        if let Some(start_time) = start_time {
            qry.start_time = Some(start_time);
        }
        if let Some(end_time) = end_time {
            qry.end_time = end_time;
        }

        // fill the struct with non-default options
        Query {
            queue_id,
            r#type,
            start_idx,
            page_size,
            ..qry
        }
    }
}

#[derive(Debug)]
pub struct MatchesRequest<'b> {
    pub headers: HeaderMap,
    pub puuid: &'b str,
    pub query: Query<'b>,
    pub matches: <Vec<String> as IntoIterator>::IntoIter,
    pub client: reqwest::blocking::Client,
}

impl<'b> Default for MatchesRequest<'b> {
    fn default() -> Self {
        Self {
            headers: reqwest::header::HeaderMap::new(),
            puuid: "",
            query: Query::default(),
            matches: vec![].into_iter(),
            client: reqwest::blocking::Client::new(),
        }
    }
}

#[allow(dead_code)]
impl<'b> MatchesRequest<'b> {
    pub fn new(
        headers: HeaderMap,
        puuid: &'b str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        queue_id: u16, // The queue type for matches, draft = 400, blind = 430, ARAM = 450
        r#type: &'b str,
        start_idx: u16,
        page_size: u16,
    ) -> Self {
        Self {
            headers,
            puuid,
            query: Query::new(start_time, end_time, queue_id, r#type, start_idx, page_size),
            matches: Default::default(),
            client: Default::default(),
        }
    }

    fn try_next(&mut self) -> Result<Option<String>, Error> {
        // see if there are any results from our matches requests
        if let Some(m) = self.matches.next() {
            return Ok(Some(m));
        }

        // format url
        let url = format!(
            "https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/{puuid}/ids",
            puuid = self.puuid
        );

        let resp = self
            .client
            .get(&url)
            .headers(self.headers.clone())
            .query(&self.query)
            .send()?
            .json::<Vec<String>>()?; // ::<MatchesResponse>()?;

        // turn our vector string response into an iterator
        self.matches = resp.into_iter();

        // increment the start_idx by the page size (count)
        self.query.start_idx += self.query.page_size;

        Ok(self.matches.next())
    }
}

impl<'b> Iterator for MatchesRequest<'b> {
    type Item = Result<String>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(s)) => Some(Ok(s)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

/////////////////////

// use serde::Serialize;

// #[derive(Serialize, Debug)]
// struct Query {
//     queue: u16,
//     start: usize,
//     count: u16,
// }

// #[derive(Debug)]
// pub struct Matches<'a> {
//     pub headers: &'a HeaderMap,
//     pub url: String,
//     pub queue: u16, // The queue type for matches, draft = 400, blind = 430, ARAM = 450
// }
//
// // Number of matches to search for
// const NUM_MATCHES_TO_SEARCH: usize = 1000;
// impl<'a> Matches<'a> {
//     pub fn get_matches(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//         println!("Retrieving all League of Legends Matches...");
//         let mut matches: Vec<String> = Vec::new();
//         for idx in (0..NUM_MATCHES_TO_SEARCH).step_by(100) {
//             // Create dynamic query per request
//             let qry: Query = Query {
//                 queue: self.queue,
//                 start: idx, // Index of where to search in match results
//                 count: 100, // How many matches to search at once
//             };
//
//             let client = reqwest::blocking::Client::new();
//             let mut resp = client
//                 .get(&self.url)
//                 .headers(self.headers.to_owned())
//                 .query(&qry)
//                 .send()?;
//
//             // Retry if we receive a 429 - too many requests
//             while resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
//                 println!("429 received... sleeping for 2 minutes");
//                 std::thread::sleep(Duration::from_secs(120)); // Sleep 2 minutes as required
//                                                               // by API
//                 resp = client
//                     .get(&self.url)
//                     .headers(self.headers.to_owned())
//                     .query(&qry)
//                     .send()?;
//             }
//
//             // Error check and process as string
//             let mut data: Vec<String> = resp.error_for_status()?.json()?;
//             println!("Processed: {:?} matches", &data);
//             matches.append(&mut data);
//         }
//
//         // Sort and deduplicate consecutive vector elements
//         matches.sort();
//         matches.dedup();
//         println!(
//             "There are {} unique matches for this queue.",
//             &matches.len()
//         );
//         Ok(matches)
//     }
// }

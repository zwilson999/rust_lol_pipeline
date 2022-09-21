use reqwest::header::HeaderMap;
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize, Debug)]
struct Query {
    queue: u16,
    start: usize,
    count: u16,
}

// Number of matches to search for
const NUM_MATCHES_TO_SEARCH: usize = 1000;

#[derive(Debug)]
pub struct Matches<'a> {
    pub headers: &'a HeaderMap,
    pub url: String,
    pub queue: u16, // The queue type for matches, draft = 400, blind = 430, ARAM = 450
}
impl<'a> Matches<'a> {
    pub fn get_matches(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut matches: Vec<String> = Vec::new();
        for idx in (0..NUM_MATCHES_TO_SEARCH).step_by(100) {
            let qry: Query = Query {
                queue: self.queue,
                start: idx, // idx of where to search in match results
                count: 100, // How many matches to search at once
            };

            let client = reqwest::blocking::Client::new();
            let mut resp = client
                .get(&self.url)
                .headers(self.headers.to_owned())
                .query(&qry)
                .send()?;
            while resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                std::thread::sleep(Duration::from_secs(120));
                resp = client
                    .get(&self.url)
                    .headers(self.headers.to_owned())
                    .query(&qry)
                    .send()?;
            }

            // Error check and process as string
            let mut data: Vec<String> = resp.error_for_status()?.json()?;
            matches.append(&mut data);
        }

        // Sort and deduplicate consecutive vector elements
        matches.sort();
        matches.dedup();
        println!(
            "There are {} unique matches for this queue.",
            &matches.len()
        );
        Ok(matches)
    }
}

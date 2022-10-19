use reqwest::header::HeaderMap;
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize, Debug)]
struct Query {
    queue: u16,
    start: usize,
    count: u16,
}

#[derive(Debug)]
pub struct Matches<'a> {
    pub headers: &'a HeaderMap,
    pub url: String,
    pub queue: u16, // The queue type for matches, draft = 400, blind = 430, ARAM = 450
}

// Number of matches to search for
const NUM_MATCHES_TO_SEARCH: usize = 1000;
impl<'a> Matches<'a> {
    pub fn get_matches(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {

        println!("Retrieving all League of Legends Matches...");
        let mut matches: Vec<String> = Vec::new();
        for idx in (0..NUM_MATCHES_TO_SEARCH).step_by(100) {
            let qry: Query = Query {
                queue: self.queue,
                start: idx, // Index of where to search in match results
                count: 100, // How many matches to search at once
            };

            let client = reqwest::blocking::Client::new();
            let mut resp = client
                .get(&self.url)
                .headers(self.headers.to_owned())
                .query(&qry)
                .send()?;

            // Retry if we receive a 429 - too many requests
            while resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                println!("429 received... sleeping for 2 minutes");
                std::thread::sleep(Duration::from_secs(120)); // Sleep 2 minutes as required
                                                                   // by API
                resp = client
                    .get(&self.url)
                    .headers(self.headers.to_owned())
                    .query(&qry)
                    .send()?;
            }

            // Error check and process as string
            let mut data: Vec<String> = resp.error_for_status()?.json()?;
            println!("Processed: {:?} matches", &data);
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

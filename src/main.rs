mod api;
mod config;
mod pipeline;

use anyhow::{Error, Result};
use clap::Parser;
use config::{get_api_keys, Config};
use pipeline::Pipeline;
use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};
// use futures::{stream, StreamExt};
// use postgres::NoTls;
// use reqwest::header::HeaderMap;
// use tokio::time::{sleep, Duration};
//
//
// fn get_summoner_puuid(
//     headers: &HeaderMap,
//     summoner_url: &str,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     // Construct Summoner and headers needed for summoner request
//     let summoner = Summoner {
//         headers: &headers,
//         url: &summoner_url,
//     };
//
//     // Get puuid which will be used to send requests for a specific summoner ID
//     let puuid: String = summoner.get_puuid().unwrap();
//     Ok(puuid)
// }
//
// fn load_matches(matches: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
//     // Read in user's postgresql password and trim extra space
//     let password = fs::read_to_string("creds/pg_password.txt")?;
//     let pwd = password.trim_end();
//
//     // Connect to postgres
//     let mut client = postgres::Client::connect(
//         format!("postgres://postgres:{pwd}@localhost/LeagueOfLegends").as_str(),
//         NoTls,
//     )?;
//
//     // Truncate matches table before inserting
//     client.execute("truncate table matches;", &[])?;
//
//     // Insert Match IDs into postgres
//     for m in matches {
//         client.execute("insert into matches (match_id) values ($1);", &[&m])?;
//         println!("Inserted match_id: {m}");
//     }
//     Ok(())
// }
//
// fn get_summoner_matches(
//     headers: &HeaderMap,
//     puuid: &str,
// ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//     // Construct Matches
//     let matches: Matches<'_> = Matches {
//         headers: &headers,
//         url: format!(
//             "https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids",
//             &puuid
//         ),
//         queue: 400,
//     };
//     let matches: Vec<String> = matches.get_matches().unwrap();
//
//     // Insert matches into postgres before returning
//     load_matches(&matches).unwrap();
//     Ok(matches)
// }
//
// fn get_match_data(
//     headers: &HeaderMap,
//     matches: Vec<String>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // Construct MatchData
//     let match_data = MatchData {
//         headers: &headers,
//         matches,
//     };
//
//     // Prepare request URLs
//     let match_data_requests: Vec<String> = match_data.prepare().unwrap();
//
//     // Build runtime before making async requests
//     let rt = tokio::runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//         .unwrap();
//
//     // Make async requests
//     rt.block_on(make_match_requests(&headers, &match_data_requests))
//         .unwrap();
//     Ok(())
// }
//
// async fn make_match_requests(
//     headers: &HeaderMap,
//     match_requests: &Vec<String>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // Create async request client to be re-used
//     let client = reqwest::Client::new();
//
//     // Construct async requests
//     let bodies = stream::iter(match_requests)
//         .map(|url| {
//             let client = &client;
//             async move { fetch(client, headers, &url).await }
//         })
//         .buffer_unordered(20);
//
//     // Check out our response bodies
//     bodies
//         .for_each(|b| async {
//             match b {
//                 Ok(b) => println!("{:?}", b),
//                 Err(e) => eprintln!("Got an error: {}", e),
//             }
//         })
//         .await;
//
//     Ok(())
// }
//
// async fn fetch(
//     client: &reqwest::Client,
//     headers: &HeaderMap,
//     match_request: &String,
// ) -> Result<Match, Box<dyn std::error::Error>> {
//     // Make initial request
//     let resp = client
//         .get(match_request)
//         .headers(headers.to_owned())
//         .send()
//         .await?;
//
//     // Sleep 1s asynchronously to avoid rate limit of 20 requests/s
//     sleep(Duration::from_secs(1)).await;
//
//     // If we hit rate limit, wait 2 minutes and retry
//     while resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
//         // Retrieve how long we need to wait before retrying the request
//         let wait = resp
//             .headers()
//             .get("Retry-After")
//             .unwrap()
//             .to_str()
//             .unwrap()
//             .parse::<u64>()
//             .unwrap();
//         println!("Waiting for {:?}", wait);
//
//         // Sleep for wait seconds
//         sleep(Duration::from_secs(wait)).await;
//
//         // Retry the request
//         let resp = client
//             .get(match_request)
//             .headers(headers.to_owned())
//             .send()
//             .await?;
//
//         // Resolve the text response
//         let txt = resp.text().await?;
//         let data = serde_json::from_str::<Match>(&txt)?;
//         println!("{:?}", data);
//         return Ok(data);
//     }
//
//     // Resolve the text response if data isn't a 429 (too many requests)
//     let txt = resp.text().await?;
//     let data = serde_json::from_str::<Match>(&txt)?;
//     println!("{:?}", data);
//     Ok(data)
// }

// run query and deserialize it into a struct

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    user: String,
    #[arg(short, long)]
    pwd: String,
    #[arg(short, long)]
    summoner: String,
    #[arg(long, default_value_t = 1338253148)]
    start: u64,
    #[arg(long, default_value_t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())]
    end: u64,
}

fn main() -> Result<(), Error> {
    let start = Instant::now();
    let args = Args::parse();

    // get api keys from db
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let account_info = get_api_keys(&args.user, &args.pwd, &args.summoner)
            .await
            .unwrap_or_else(|err| {
                eprintln!("ERROR: could not obtain credentials from db... check user args. {err}");
                std::process::exit(1);
            });
        println!("{:?}", account_info);

        let config = Config::build(account_info, args.start, args.end);

        // create pipeline and run it
        // let pipeline = Pipeline::new(config);
        // pipeline.run();

        println!("INFO: Program finished in: {:.2?}", start.elapsed());
    });

    Ok(())
}

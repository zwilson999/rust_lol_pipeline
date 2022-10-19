use std::fs;
use std::time::Instant;
mod api;
use crate::api::{MatchData, Matches, Summoner, Match};
use reqwest::header::HeaderMap;
use tokio::time::{sleep, Duration};
use futures::{stream, StreamExt};

fn construct_headers(api_key: &str) -> HeaderMap {
    
    // Construct HeaderMap for making a request to Riot League of Legends Summoner API
    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("X-Riot-Token", api_key.parse().unwrap());
    return headers;
}

fn get_summoner_puuid(headers: &HeaderMap, summoner_url: &str) -> Result<String, Box<dyn std::error::Error>> {

    // Construct Summoner and headers needed for summoner request
    let summoner = Summoner {
        headers: &headers,
        url: &summoner_url,
    };

    // Get puuid which will be used to send requests for a specific summoner ID
    let puuid: String = summoner.get_puuid()
                                .unwrap();
    Ok(puuid)
}

fn get_summoner_matches(headers: &HeaderMap, puuid: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    
    // Construct Matches
    let matches = Matches {
        headers: &headers,
        url: format!(
            "https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids",
            &puuid
        ),
        queue: 400,
    };
    let matches: Vec<String> = matches
        .get_matches()
        .unwrap();
    Ok(matches)
}

fn get_match_data(headers: &HeaderMap, matches: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    // Construct MatchData
    let match_data = MatchData {
        headers: &headers,
        matches,
    };

    // Prepare request URLs
    let match_data_requests: Vec<String> = match_data.prepare().unwrap();

    // Build runtime before making async requests
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // Make async requests
    rt.block_on(make_match_requests(&headers, &match_data_requests)).unwrap();
    Ok(())
}

async fn make_match_requests(headers: &HeaderMap, match_requests: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    // Create async request client to be re-used
    let client = reqwest::Client::new();
        
    // Construct async requests
    let bodies = stream::iter(match_requests)
        .map(|url| {
            let client = &client;
            async move {  
                fetch(client, headers, &url).await
            }
        })
        .buffer_unordered(20);

    // Check out our response bodies
    bodies.for_each(|b| async {
        match b {
            Ok(b) => println!("{:?}", b),
            Err(e) => eprintln!("Got an error: {}", e),
        }
    })
    .await;

    Ok(())
}

async fn fetch(client: &reqwest::Client, headers: &HeaderMap, match_request: &String) -> Result<Match, Box<dyn std::error::Error>> {

    // Make initial request
    let resp = client.get(match_request)
         .headers(headers.to_owned())
         .send()
         .await?;

    // Sleep 1s asynchronously to avoid rate limit of 20 requests/s
    sleep(Duration::from_secs(1)).await;

    // If we hit rate limit, wait 2 minutes and retry
    while resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {

        // Retrieve how long we need to wait before retrying the request
        let wait = resp
            .headers()
            .get("Retry-After")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        println!("Waiting for {:?}", wait);

        // Sleep for wait seconds
        sleep(Duration::from_secs(wait)).await;

        // Retry the request
        let resp = client
            .get(match_request)
            .headers(headers.to_owned())
            .send()
            .await?;
        // Resolve the text response 
        let txt = resp.text().await?;
        let data = serde_json::from_str::<Match>(&txt)?; 
        println!("{:?}", data);
        return Ok(data);
    } 

    // Resolve the text response if data isn't a 429 (too many requests) 
    let txt = resp.text().await?;
    let data = serde_json::from_str::<Match>(&txt)?; 
    println!("{:?}", data);
    Ok(data)
    
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Capture program start time
    let start = Instant::now();

    // Read in Riot API Key and construct headers from an external file or location (possibly provided by user)
    let api_key: String = fs::read_to_string("creds/api_key.txt")?.parse()?;
    let headers: HeaderMap = construct_headers(&api_key);

    // Create unique summoner_url from user's summoner entry
    let mut summoner_url: String = String::from("https://na1.api.riotgames.com/lol/summoner/v4/summoners/by-name/");
    let summoner: &str = "Epoetin Alfa"; // Will receive from user args in later version
    summoner_url.push_str(summoner);

    // Can set queue types from user args later
    //let queue_type: u16 = 400;

    // Get Summoner's Puuid
    let puuid: String = get_summoner_puuid(&headers, &summoner_url).unwrap();
    println!("{}", puuid);

    // Get Summoner's Matches
    let matches: Vec<String> = get_summoner_matches(&headers, &puuid).unwrap();

    get_match_data(&headers, matches).unwrap();

    // Print runtime of program
    println!("Elapsed: {:.2?}", start.elapsed());
    Ok(())
}

use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_CHARSET, ACCEPT_LANGUAGE};

#[derive(Debug)]
pub struct MatchRequest<'c> {
    pub api_key: &'c str,
    pub url: &'c str,
    pub match_id: &'c str,
}

impl<'c> Default for MatchRequest<'c> {
    fn default() -> Self {
        Self {
            api_key: "",
            url: "",
            match_id: "",
        }
    }
}

impl<'c> MatchRequest<'c> {
    pub fn new(api_key: &'c str, url: &'c str, match_id: &'c str) -> Self {
        Self {
            api_key,
            url,
            match_id,
        }
    }

    fn create_headers(self) -> HeaderMap {
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
    /*
    pub fn prepare(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // Construct a vector of Match Data Requests
        let match_data_requests: Vec<String> = self
            .matches
            .iter()
            .map(|x| {
                format!(
                    "https://americas.api.riotgames.com/lol/match/v5/matches/{m}",
                    m = x
                )
            })
            .collect();

        println!(
            "There are: {} match requests to make..",
            match_data_requests.len()
        );
        Ok(match_data_requests)
    }
    */
}

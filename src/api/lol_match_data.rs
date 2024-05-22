use reqwest::header::HeaderMap;

#[derive(Debug)]
pub struct MatchRequest<'a> {
    pub headers: &'a HeaderMap,
    pub matches: Vec<String>,
}

impl<'a> MatchRequest<'a> {
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
}

use reqwest::header::HeaderMap;
use serde::Deserialize;
 
#[derive(Deserialize)]
struct Puuid 
{
    puuid: String,
}

#[derive(Debug)]
pub struct Summoner<'a> 
{
    pub headers: &'a HeaderMap,
    pub url: &'a str,
}

impl<'a> Summoner<'a> 
{
    pub fn get_puuid(&self) -> Result<String, Box<dyn std::error::Error>> 
    {
        // Make request to get PUUID from endpoint
        let client = reqwest::blocking::Client::new();
        let resp = client.get(self.url)
                         .headers(self.headers.to_owned())
                         .send()?;

        // Deserialize puuid into struct
        let puuid = resp.error_for_status()?.json::<Puuid>()?;
        println!("Successfully authenticated and retrieved player PUUID.");
        Ok(puuid.puuid)
    }
}

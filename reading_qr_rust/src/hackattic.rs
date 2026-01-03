use reqwest;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("network error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("environment variable error: {0}")]
    Env(#[from] std::env::VarError),
}


#[derive(Deserialize, Debug)]
struct ChallengeResponse {
    image_url: String,
}

pub fn get_qr_code() -> Result<String, Error> {
    let token = std::env::var("HACKATTIC_TOKEN").map_err(Error::Env)?;
    let url = format!("https://hackattic.com/challenges/reading_qr/problem?access_token={}", token);
    
    let resp: ChallengeResponse = reqwest::blocking::get(url)?.json()?;
    
    Ok(resp.image_url)
}

pub fn post_solution() -> Result<String, Error> {
    Ok(String::from(""))
}

use reqwest;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("network error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("environment variable error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("utf8 conversion error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

#[derive(Deserialize, Debug)]
struct ChallengeResponse {
    image_url: String,
}

pub fn get_qr_code() -> Result<String, Error> {
    let token = std::env::var("HACKATTIC_TOKEN").map_err(Error::Env)?;
    let url = format!(
        "https://hackattic.com/challenges/reading_qr/problem?access_token={}",
        token
    );

    let resp: ChallengeResponse = reqwest::blocking::get(url)?.json()?;

    Ok(resp.image_url)
}

pub fn post_solution(solution: &str) -> Result<String, Error> {
    let token = std::env::var("HACKATTIC_TOKEN").map_err(Error::Env)?;
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://hackattic.com/challenges/reading_qr/solve?access_token={}",
        token
    );
    let resp = client.post(url).json(&json!({"code": solution})).send()?;

    format_submission_response(resp)
}

fn format_submission_response(resp: reqwest::blocking::Response) -> Result<String, Error> {
    let status = resp.status();
    let body_bytes = resp.bytes()?;
    let body = std::str::from_utf8(&body_bytes)?;
    Ok(format!("{} {}", status, body))
}

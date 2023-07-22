
use std::env;

use reqwest::blocking::{RequestBuilder, Response};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GenericResponse<T> {
    data: T,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AgentDetails {
    #[serde(alias = "accountId")]
    account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i32,
    #[serde(alias = "startingFaction")]
    pub starting_faction: String,
}

pub fn fetch_agent_details() -> AgentDetails {
    let resp = send_get("https://api.spacetraders.io/v2/my/agent");
    let agent_details: GenericResponse<AgentDetails> = serde_json::from_str(&resp).unwrap();
    agent_details.data
}

fn send_get(url: &str) -> String {
    let client = reqwest::blocking::Client::new();
    match send_with_header(client.get(url)) {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    }
}

fn send_with_header(req: RequestBuilder) -> Result<Response, reqwest::Error> {

    req.header("Authorization", format!("Bearer {}", get_api_key())).send()
}

fn get_api_key() -> String {
    env::var("SPACE_TRADERS_API_KEY").unwrap()
}
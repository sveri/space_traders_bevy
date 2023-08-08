use std::env;

use bevy::app::AppLabel;
use bevy::prelude::Component;
use reqwest::blocking::{RequestBuilder, Response};

use serde::{Deserialize, Serialize};

use crate::game::waypoint::components::Waypoints;

#[derive(Debug, Deserialize)]
pub(crate) struct GenericResponse<T> {
    pub(crate) data: T,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct AgentDetails {
    // #[serde(alias = "accountId")]
    // account_id: String,
    pub(crate) symbol: String,
    pub(crate) headquarters: String,
    pub(crate) credits: i32,
    #[serde(alias = "startingFaction")]
    pub(crate) starting_faction: String,
}

impl AgentDetails {
    pub(crate) fn get_headquarter_system_symbol(&self) -> String {
        self.headquarters[0..self.headquarters.rfind('-').unwrap()].to_string()
    }
}

#[derive(Serialize, Debug)]
struct Navigate {
    #[serde(rename(serialize = "waypointSymbol"))]
    waypoint_symbol: String,
}

pub(crate) fn fetch_agent_details() -> AgentDetails {
    let resp = send_get("https://api.spacetraders.io/v2/my/agent");
    let agent_details: GenericResponse<AgentDetails> = serde_json::from_str(&resp).unwrap();
    agent_details.data
}

pub(crate) fn fetch_waypoints(system_symbol: &str) -> Waypoints {
    let resp = send_get(format!("https://api.spacetraders.io/v2/systems/{}/waypoints", system_symbol).as_str());
    let response: GenericResponse<Waypoints> = serde_json::from_str(&resp).unwrap();
    response.data
}

pub(crate) fn orbit_ship(ship_symbol: &str) -> String {
    let resp = send_post(format!("https://api.spacetraders.io/v2/my/ships/{}/orbit", ship_symbol).as_str(), "".to_string());
    resp
}

pub(crate) fn dock_ship(ship_symbol: &str) -> String {
    let resp = send_post(format!("https://api.spacetraders.io/v2/my/ships/{}/dock", ship_symbol).as_str(), "".to_string());
    resp
}

pub(crate) fn move_ship(ship_symbol: &str, target_waypoint: String) -> String {
    let navigate = Navigate {
        waypoint_symbol: target_waypoint,
    };
    let resp = send_post(
        format!("https://api.spacetraders.io/v2/my/ships/{}/navigate", ship_symbol).as_str(),
        serde_json::to_string(&navigate).unwrap(),
    );
    resp
}

pub(crate) fn send_get(url: &str) -> String {
    let client = reqwest::blocking::Client::new();
    match send_with_header(client.get(url)) {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    }
}

pub(crate) fn send_post(url: &str, body: String) -> String {
    let client = reqwest::blocking::Client::new();
    match send_with_header(client.post(url).body(body)) {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    }
}

fn send_with_header(req: RequestBuilder) -> Result<Response, reqwest::Error> {
    req.header("Authorization", format!("Bearer {}", get_api_key()))
        .header("Content-Type", "application/json")
        .send()
}

fn get_api_key() -> String {
    match env::var("SPACE_TRADERS_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("Please set the SPACE_TRADERS_API_KEY environment variable."),
    }
}

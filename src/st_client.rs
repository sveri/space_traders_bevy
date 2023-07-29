use std::env;

use bevy::{app::AppLabel, prelude::Component};
use reqwest::blocking::{RequestBuilder, Response};

use serde::{Deserialize, Serialize};

use crate::util::Point;

#[derive(Debug, Deserialize)]
pub(crate) struct GenericResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AgentDetails {
    // #[serde(alias = "accountId")]
    // account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i32,
    #[serde(alias = "startingFaction")]
    pub starting_faction: String,
}

impl AgentDetails {
    pub fn get_headquarter_system_symbol(&self) -> String {
        self.headquarters[0..self.headquarters.rfind('-').unwrap()].to_string()
    }
}

pub type Waypoints = Vec<Waypoint>;

#[derive(Debug, Deserialize, Component, Clone)]
pub struct Waypoint {
    #[serde(alias = "systemSymbol")]
    pub system_symbol: String,
    pub symbol: String,
    pub x: f32,
    pub y: f32,
    traits: Vec<WaypointTrait>,
}

impl Waypoint {
    pub fn get_display_size(&self) -> f32 {
        5.
    }

    // pub fn get_position(&self) -> Point {
    //     Point {x: self.x, y: self.y }
    // }

    pub fn in_bounds(&self, x: f32, y: f32) -> bool {
        (x - self.x).powf(2.) + (y - self.y).powf(2.) <= self.get_display_size().powf(2.)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct WaypointTrait {
    name: String,
}

impl Waypoint {
    pub fn get_traits(&self) -> String {
        let ts = &self.traits;
        ts.iter().map(|t| t.name.clone()).collect::<Vec<String>>().join(", ")
    }
}

#[derive(Serialize, Debug)]
struct Navigate {
    #[serde(rename(serialize = "waypointSymbol"))]
    waypoint_symbol: String    
}

pub fn fetch_agent_details() -> AgentDetails {
    let resp = send_get("https://api.spacetraders.io/v2/my/agent");
    let agent_details: GenericResponse<AgentDetails> = serde_json::from_str(&resp).unwrap();
    agent_details.data
}

pub fn fetch_waypoints(system_symbol: &str) -> Waypoints {
    let resp = send_get(format!("https://api.spacetraders.io/v2/systems/{}/waypoints", system_symbol).as_str());
    let response: GenericResponse<Waypoints> = serde_json::from_str(&resp).unwrap();
    response.data
}

pub fn orbit_ship(ship_symbol: &str) -> String {
    let resp = send_post(format!("https://api.spacetraders.io/v2/my/ships/{}/orbit", ship_symbol).as_str(), "".to_string());
    resp
}

pub fn move_ship(ship_symbol: &str, target_waypoint: String) -> String {
    let navigate = Navigate {
        waypoint_symbol: target_waypoint
    };
    dbg!(serde_json::to_string(&navigate).unwrap());
    let resp = send_post(format!("https://api.spacetraders.io/v2/my/ships/{}/navigate", ship_symbol).as_str(), serde_json::to_string(&navigate).unwrap());
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
    req.header("Authorization", format!("Bearer {}", get_api_key())).header("Content-Type", "application/json").send()
}

fn get_api_key() -> String { env::var("SPACE_TRADERS_API_KEY").unwrap() }
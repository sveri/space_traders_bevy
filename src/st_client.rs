use std::{env, fmt::Display};

use anyhow::{anyhow, Context, Result};
use reqwest::{
    blocking::{RequestBuilder, Response},
    StatusCode,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::game::{
    components::Market,
    ship::components::{Nav, NavWrapper, Ship, BestItemToTrade, PurchaseSellResponse, FlightStatus},
    waypoint::components::Waypoints,
};

#[derive(thiserror::Error, Debug)]
enum SpaceTradersApiError {
    #[error("Unwrapping json failed: {0}")]
    JsonUnwrapError(#[from] serde_json::Error),
    #[error("Bad Request: {0}")]
    BadRequestError(String),
    // JsonUnwrapError(String),
}

#[derive(Debug, Deserialize)]
pub(crate) struct BadRequestResponse {
    pub(crate) error: BadRequestErrorError,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BadRequestErrorError {
    pub(crate) message: String,
    pub(crate) code: i32,
}

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

#[derive(Serialize, Debug)]
struct PurchaseSell {
    symbol: String,
    units: i32
}

pub(crate) fn fetch_agent_details() -> AgentDetails {
    let resp = send_get("https://api.spacetraders.io/v2/my/agent");
    let agent_details: GenericResponse<AgentDetails> = serde_json::from_str(&resp.unwrap()).unwrap();
    agent_details.data
}

pub(crate) fn fetch_waypoints(system_symbol: &str) -> Result<Waypoints> {
    let resp: Waypoints =
        send_get_with_response_type(format!("https://api.spacetraders.io/v2/systems/{}/waypoints", system_symbol).as_str())?;
    Ok(resp)
}

pub(crate) fn orbit_ship(ship_symbol: &str) -> Result<String> {
    let resp = send_post(format!("https://api.spacetraders.io/v2/my/ships/{}/orbit", ship_symbol).as_str(), "".to_string());
    Ok(resp)
}

pub(crate) fn dock_ship(ship_symbol: &str) -> Result<String> {
    let resp = send_post(format!("https://api.spacetraders.io/v2/my/ships/{}/dock", ship_symbol).as_str(), "".to_string());
    Ok(resp)
}

// pub(crate) fn move_ship(ship: &mut Ship, target_waypoint: String) -> Result<Nav> {
pub(crate) fn move_ship(ship: &Ship, target_waypoint: String) -> Result<Nav> {
    let navigate = Navigate {
        waypoint_symbol: target_waypoint,
    };

    if ship.is_docked() {
        // ship.nav.status = FlightStatus::IN_ORBIT;
        orbit_ship(&ship.symbol)?;
    }

    match send_post_with_error(
        format!("https://api.spacetraders.io/v2/my/ships/{}/navigate", ship.symbol).as_str(),
        serde_json::to_string(&navigate).unwrap(),
    ) {
        Ok(resp) => {
            let nav_wrapper: GenericResponse<NavWrapper> = serde_json::from_str(&resp)?;
            Ok(nav_wrapper.data.nav)
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn buy_items(ship: &Ship, item: &BestItemToTrade) -> Result<PurchaseSellResponse> {
    let purchase_body = PurchaseSell {
        symbol: item.item.clone(),
        units: 10,
    };

    if ship.is_in_orbit() {
        dock_ship(&ship.symbol)?;
    }

    match send_post_with_error(
        format!("https://api.spacetraders.io/v2/my/ships/{}/purchase", ship.symbol).as_str(),
        serde_json::to_string(&purchase_body).unwrap(),
    ) {
        Ok(resp) => {
            let nav_wrapper: GenericResponse<PurchaseSellResponse> = serde_json::from_str(&resp)?;
            Ok(nav_wrapper.data)
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn sell_items(ship: &Ship, sell_symbol: String, units: i32) -> Result<PurchaseSellResponse> {
    let sell_body = PurchaseSell {
        symbol: sell_symbol,
        units,
    };

    if ship.is_in_orbit() {
        dock_ship(&ship.symbol)?;
    }

    match send_post_with_error(
        format!("https://api.spacetraders.io/v2/my/ships/{}/sell", ship.symbol).as_str(),
        serde_json::to_string(&sell_body).unwrap(),
    ) {
        Ok(resp) => {
            let nav_wrapper: GenericResponse<PurchaseSellResponse> = serde_json::from_str(&resp)?;
            Ok(nav_wrapper.data)
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn get_market_data(system_symbol: &str, waypoint_symbol: &str) -> Result<Market> {
    let resp: Market = send_get_with_response_type(
        format!("https://api.spacetraders.io/v2/systems/{}/waypoints/{}/market", system_symbol, waypoint_symbol).as_str(),
    )?;
    Ok(resp)
}

pub(crate) fn send_get_with_response_type<T: DeserializeOwned>(url: &str) -> Result<T> {
    let client = reqwest::blocking::Client::new();
    let r = send_with_header(client.get(url))?.text()?;
    serde_json::from_str::<GenericResponse<T>>(&r)
        .map_or_else(|e| Err(anyhow!(SpaceTradersApiError::JsonUnwrapError(e))), |resp| Ok(resp.data))
}

pub(crate) fn send_get(url: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let r = send_with_header(client.get(url))?.text()?;
    Ok(r)
}

pub(crate) fn send_post(url: &str, body: String) -> String {
    let client = reqwest::blocking::Client::new();
    match send_with_header(client.post(url).body(body)) {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    }
}

pub(crate) fn send_post_with_error(url: &str, body: String) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    match send_with_header(client.post(url).body(body)) {
        Ok(resp) => {
            if &resp.status() == &StatusCode::BAD_REQUEST {
                let bad_request_response: BadRequestResponse = serde_json::from_str(&resp.text().unwrap()).unwrap();
                println!("Error: {}", &bad_request_response.error.message);
                Err(anyhow!(SpaceTradersApiError::BadRequestError(bad_request_response.error.message)))
            } else {
                Ok(resp.text().unwrap())
            }
        }
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

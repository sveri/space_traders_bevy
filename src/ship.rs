use serde::Deserialize;
use crate::st_client;

pub type Ships = Vec<Ship>;

#[derive(Debug, Deserialize)]
pub struct Ship {
    pub symbol: String,
    pub crew: Crew,
    pub fuel: Fuel,
    pub nav: Nav,
    // pub x: i32,
    // pub y: i32,
}

#[derive(Debug, Deserialize)]
pub struct Nav {
    pub status: String,
    #[serde(alias = "waypointSymbol")]
    pub waypoint_symbol: String,    
}

#[derive(Debug, Deserialize)]
pub struct Crew {
    pub current: i32,
    pub capacity: i32,
    pub required: i32,
    pub morale: i32,
    pub wages: i32,
}

#[derive(Debug, Deserialize)]
pub struct Fuel {
    pub current: i32,
    pub capacity: i32,
}

#[derive(Debug, Deserialize)]
pub struct Cargo {
    pub capacity: i32,
    pub units: i32,
}


pub fn fetch_my_ships() -> Ships {
    let resp = st_client::send_get("https://api.spacetraders.io/v2/my/ships");
    let data_wrapper: st_client::GenericResponse<Ships> = serde_json::from_str(&resp).unwrap();
    data_wrapper.data
}
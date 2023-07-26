use crate::{st_client, util::Point};
use bevy::{prelude::Component};
use serde::Deserialize;
pub type Ships = Vec<Ship>;

#[derive(Debug, Deserialize, Component, Clone)]
pub struct Ship {
    pub symbol: String,
    pub crew: Crew,
    pub fuel: Fuel,
    pub nav: Nav,
}

impl Ship {
    pub fn get_display_size(&self) -> (f32, f32) {
        (2., 4.)
    }

    pub fn get_position(&self) -> Point {
        Point {x: self.nav.route.departure.x, y: self.nav.route.departure.y }
    }

    pub fn in_bounds(&self, x: f32, y: f32) -> bool {
        x <= self.get_position().x + self.get_display_size().0 / 2. 
        && x >= self.get_position().x - self.get_display_size().0 / 2.
        && y <= self.get_position().y + self.get_display_size().1 / 2.
        && y >= self.get_position().y - self.get_display_size().1 / 2.
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Nav {
    pub status: String,
    #[serde(alias = "waypointSymbol")]
    pub waypoint_symbol: String,
    pub route: Route,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Route {
    pub departure: Departure,
    pub destination: Destination,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Departure {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Destination {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Crew {
    pub current: i32,
    pub capacity: i32,
    pub required: i32,
    pub morale: i32,
    pub wages: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Fuel {
    pub current: i32,
    pub capacity: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cargo {
    pub capacity: i32,
    pub units: i32,
}

pub fn fetch_my_ships() -> Ships {
    let resp = st_client::send_get("https://api.spacetraders.io/v2/my/ships");
    let data_wrapper: st_client::GenericResponse<Ships> = serde_json::from_str(&resp).unwrap();
    data_wrapper.data
}

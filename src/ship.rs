use crate::{st_client, util::Point};
use bevy::{prelude::Component};
use serde::Deserialize;
pub(crate) type Ships = Vec<Ship>;

#[derive(Debug, Deserialize, Component, Clone)]
pub(crate) struct Ship {
    pub(crate) symbol: String,
    pub(crate) crew: Crew,
    pub(crate) fuel: Fuel,
    pub(crate) nav: Nav,
}

impl Ship {
    pub(crate) fn get_display_size(&self) -> (f32, f32) {
        (2., 4.)
    }

    pub(crate) fn get_position(&self) -> Point {
        Point {x: self.nav.route.departure.x, y: self.nav.route.departure.y }
    }

    pub(crate) fn in_bounds(&self, x: f32, y: f32) -> bool {
        x <= self.get_position().x + self.get_display_size().0 / 2. 
        && x >= self.get_position().x - self.get_display_size().0 / 2.
        && y <= self.get_position().y + self.get_display_size().1 / 2.
        && y >= self.get_position().y - self.get_display_size().1 / 2.
    }
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Nav {
    pub(crate) status: String,
    #[serde(alias = "waypointSymbol")]
    pub(crate) waypoint_symbol: String,
    pub(crate) route: Route,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Route {
    pub(crate) departure: Departure,
    pub(crate) destination: Destination,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Departure {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Destination {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Crew {
    pub(crate) current: i32,
    pub(crate) capacity: i32,
    pub(crate) required: i32,
    pub(crate) morale: i32,
    pub(crate) wages: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Fuel {
    pub(crate) current: i32,
    pub(crate) capacity: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Cargo {
    pub(crate) capacity: i32,
    pub(crate) units: i32,
}

pub(crate) fn fetch_my_ships() -> Ships {
    let resp = st_client::send_get("https://api.spacetraders.io/v2/my/ships");
    let data_wrapper: st_client::GenericResponse<Ships> = serde_json::from_str(&resp).unwrap();
    data_wrapper.data
}

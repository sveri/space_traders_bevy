#![allow(dead_code)]

use bevy::prelude::*;
use serde::Deserialize;
use chrono::{DateTime, Duration, Utc};

use crate::util::Point;


#[derive(Component)]
pub(crate) struct ShipRepresentation;


#[derive(Component)]
pub(super) struct ShipComponent;


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
        if self.nav.route.departure.symbol == self.nav.route.destination.symbol {
            return Point {x: self.nav.route.departure.x, y: self.nav.route.departure.y };
        } else {
            let utc: DateTime<Utc> = Utc::now();
            let arrival_time: DateTime<Utc> = self.nav.route.arrival.parse::<DateTime<Utc>>().unwrap();
            if (utc - arrival_time).num_milliseconds() > 0 {
                return Point {x: self.nav.route.destination.x, y: self.nav.route.destination.y };
            }
        }
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
    pub(crate) arrival: String,
    #[serde(alias = "departureTime")]
    pub(crate) departure_time: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Departure {
    pub(crate) symbol: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Destination {
    pub(crate) symbol: String,
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
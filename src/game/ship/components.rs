#![allow(dead_code)]

use bevy::prelude::*;
use serde::Deserialize;
use chrono::{DateTime, Utc};

use crate::util::Point;


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
        // ship arrivale and destination are the same
        if self.nav.route.departure.symbol == self.nav.route.destination.symbol {
            Point {x: self.nav.route.departure.x, y: self.nav.route.departure.y }
        } else {
            let utc: DateTime<Utc> = Utc::now();
            let arrival_time: DateTime<Utc> = self.nav.route.arrival.parse::<DateTime<Utc>>().unwrap();

            // ship arrived at destination
            if (utc - arrival_time).num_milliseconds() > 0 {
                Point {x: self.nav.route.destination.x, y: self.nav.route.destination.y }
            } 
            // ship is moving from departure to destination
            else {
                Point {x: (self.nav.route.departure.x + self.nav.route.destination.x) / 2., y: (self.nav.route.departure.y + self.nav.route.destination.y) / 2. }

            }
        }
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

impl std::fmt::Display for Fuel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.current, self.capacity)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Cargo {
    pub(crate) capacity: i32,
    pub(crate) units: i32,
}
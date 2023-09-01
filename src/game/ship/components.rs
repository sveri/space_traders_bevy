#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]

use std::fmt::Display;

use bevy::prelude::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::Value;

use crate::game::components::Transaction;

// #[derive(Debug, Deserialize, Component, Clone)]
// pub(crate) struct ShipWrapper {
//     ship: Ship,
//     state: String
// }

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ShipStateEnum {
    Idle,
    Purchase,
    Sell,
    Autotrade,
}

#[derive(Debug, Component, Clone)]
pub(crate) struct ShipState {
    pub(crate) state: ShipStateEnum,
    pub(crate) item_to_purchase: Option<String>,
    pub(crate) to_purchase_waypoint: Option<String>,
    pub(crate) item_to_sell: Option<String>,
    pub(crate) to_sell_waypoint: Option<String>,
}

impl ShipState {
    pub(crate) fn new() -> Self {
        ShipState {
            state: ShipStateEnum::Idle,
            item_to_purchase: None,
            to_purchase_waypoint: None,
            item_to_sell: None,
            to_sell_waypoint: None,
        }
    }

    pub(crate) fn is_idle(&self) -> bool { self.state == ShipStateEnum::Idle }

    pub(crate) fn has_to_find_new_item_to_purchase(&self) -> bool {
        if self.item_to_purchase.is_none() || self.item_to_sell.is_none() {
            return true;
        }
        false
    }

    pub(crate) fn has_to_dock(&self, ship: &Ship) -> bool {
        if ship.is_docked() {
            return false;
        }

        if let Some(waypoint) = &self.to_purchase_waypoint {
            if ship.get_current_waypoint() == waypoint.clone() {
                return true;
            }
        }
        if let Some(waypoint) = &self.to_sell_waypoint {
            if ship.get_current_waypoint() == waypoint.clone() {
                return true;
            }
        }
        false
    }
}

pub(crate) type Ships = Vec<Ship>;

#[derive(Reflect, Debug, Deserialize, Component, Clone)]
pub(crate) struct Ship {
    pub(crate) symbol: String,
    pub(crate) crew: Crew,
    pub(crate) fuel: Fuel,
    pub(crate) nav: Nav,
    pub(crate) cargo: Cargo,
}

impl Ship {
    pub(crate) fn get_display_size(&self) -> (f32, f32) { (2., 4.) }

    pub(crate) fn has_arrived_at_destionation(&self) -> bool {
        let utc: DateTime<Utc> = Utc::now();
        let arrival_time: DateTime<Utc> = self.nav.route.arrival.parse::<DateTime<Utc>>().unwrap();
        if (utc - arrival_time).num_milliseconds() > 0 {
            tracing::trace!("ship {} arrived at destination", self.symbol);
            return true;
        }
        false
    }

    pub(crate) fn get_position(&self) -> Vec3 {
        // ship arrival and destination are the same
        if self.nav.route.departure.symbol == self.nav.route.destination.symbol {
            tracing::trace!("ship {}, is at its destination", self.symbol);
            Vec3 {
                x: self.nav.route.departure.x,
                y: self.nav.route.departure.y,
                z: 1.0,
            }
        } else {
            // ship arrived at destination
            if self.has_arrived_at_destionation() {
                tracing::trace!("ship {} arrived at destination", self.symbol);
                Vec3 {
                    x: self.nav.route.destination.x,
                    y: self.nav.route.destination.y,
                    z: 1.0,
                }
            }
            // ship is moving from departure to destination
            else {
                tracing::trace!("ship is moving to destination");
                Vec3 {
                    x: (self.nav.route.departure.x + self.nav.route.destination.x) / 2.,
                    y: (self.nav.route.departure.y + self.nav.route.destination.y) / 2.,
                    z: 1.0,
                }
            }
        }
    }

    pub(crate) fn get_transform(&self) -> Transform { Transform::from_translation(self.get_position()) }

    pub(crate) fn is_in_transit(&self) -> bool { self.nav.status == FlightStatus::IN_TRANSIT }

    pub(crate) fn has_cargo(&self) -> bool { self.cargo.units > 0 }

    pub(crate) fn is_in_orbit(&self) -> bool { self.nav.status == FlightStatus::IN_ORBIT }

    pub(crate) fn is_docked(&self) -> bool { self.nav.status == FlightStatus::DOCKED }

    pub(crate) fn get_current_waypoint(&self) -> String { self.nav.waypoint_symbol.clone() }

    pub(crate) fn must_refuel(&self) -> bool { self.fuel.current < 200 }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct NavWrapper {
    pub(crate) nav: Nav,
}

#[derive(Reflect, Debug, Deserialize, Clone)]

pub(crate) struct Nav {
    pub(crate) status: FlightStatus,
    #[serde(alias = "systemSymbol")]
    pub(crate) system_symbol: String,
    #[serde(alias = "waypointSymbol")]
    pub(crate) waypoint_symbol: String,
    #[serde(alias = "flightMode")]
    pub(crate) flight_mode: FlightMode,
    pub(crate) route: Route,
}

#[derive(Reflect, Debug, Deserialize, Clone, PartialEq)]
pub(crate) enum FlightStatus {
    IN_TRANSIT,
    IN_ORBIT,
    DOCKED,
}

impl Display for FlightStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlightStatus::IN_TRANSIT => write!(f, "In Transit"),
            FlightStatus::IN_ORBIT => write!(f, "In Orbit"),
            FlightStatus::DOCKED => write!(f, "Docked"),
        }
    }
}

#[derive(Reflect, Debug, Deserialize, Clone, PartialEq)]
pub(crate) enum FlightMode {
    DRIFT,
    STEALTH,
    CRUISE,
    BURN,
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub(crate) struct Route {
    pub(crate) departure: Departure,
    pub(crate) destination: Destination,
    pub(crate) arrival: String,
    #[serde(alias = "departureTime")]
    pub(crate) departure_time: String,
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub(crate) struct Departure {
    pub(crate) symbol: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub(crate) struct Destination {
    pub(crate) symbol: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub(crate) struct Crew {
    pub(crate) current: i32,
    pub(crate) capacity: i32,
    pub(crate) required: i32,
    pub(crate) morale: i32,
    pub(crate) wages: i32,
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub(crate) struct Fuel {
    pub(crate) current: i32,
    pub(crate) capacity: i32,
}

impl std::fmt::Display for Fuel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}/{}", self.current, self.capacity) }
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub(crate) struct Cargo {
    pub(crate) capacity: i32,
    pub(crate) units: i32,
    inventory: Vec<Inventory>,
}

impl Cargo {
    pub(crate) fn get_inventory(&self) -> Vec<Inventory> { self.inventory.clone() }

    pub(crate) fn set_inventory(&mut self, new_inventory: Vec<Inventory>) { self.inventory = new_inventory.clone(); }

    pub(crate) fn add_units(&mut self, units: i32) { self.units += units; }

    pub(crate) fn add_inventory_item(&mut self, item: Inventory) { self.inventory.push(item); }

    pub(crate) fn space_available(&self) -> i32 { self.capacity - self.units }
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub(crate) struct Inventory {
    pub(crate) symbol: String,
    pub(crate) units: i32,
}

#[derive(Debug)]
pub(crate) struct BestItemToTrade {
    pub(crate) item: String,
    pub(crate) purchase_waypoint: String,
    pub(crate) sell_waypoint: String,
    pub(crate) purchase_price: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PurchaseSellResponse {
    pub(crate) transaction: Transaction,
}

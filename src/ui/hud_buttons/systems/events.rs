use bevy::prelude::*;
use bevy::text::Text;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::Click;
use bevy_mod_picking::prelude::Pointer;
// use bevy::prelude::Eve

use crate::game::components::Market;
use crate::game::ship::components::FlightStatus;
use crate::game::ship::components::NavWrapper;
use crate::game::ship::components::Ship;
use crate::game::ship::components::ShipState;
use crate::game::ship::components::ShipStateEnum;
use crate::game::ship::systems::events::ShipSelected;
use crate::game::waypoint::components::Waypoint;
use crate::st_client;
use crate::st_client::GenericResponse;
use crate::ui::controls::components::SelectedShip;
use crate::ui::controls::components::SelectedWaypoint;
use crate::ui::hud::components::ErrorText;

// finally was able to use events with bevy 0.11 and patched bevy_mod_picking|_events
// see here for an example: https://github.com/chriamue/flyconomy/blob/53d5b1d91aa39b6b97ff348f22ec35e0f1152a1f/src/game/aerodrome.rs#L33
#[derive(Event, Component, Debug)]
pub(crate) struct DockClicked;

impl From<ListenerInput<Pointer<Click>>> for DockClicked {
    fn from(_click_event: ListenerInput<Pointer<Click>>) -> Self { DockClicked }
}

#[derive(Event, Component, Debug)]
pub(crate) struct OrbitClicked;

impl From<ListenerInput<Pointer<Click>>> for OrbitClicked {
    fn from(_click_event: ListenerInput<Pointer<Click>>) -> Self { OrbitClicked }
}

#[derive(Event, Component, Debug)]
pub(crate) struct MoveShipClicked;

impl From<ListenerInput<Pointer<Click>>> for MoveShipClicked {
    fn from(_click_event: ListenerInput<Pointer<Click>>) -> Self { MoveShipClicked }
}

#[derive(Event, Component, Debug)]
pub(crate) struct GetMarketClicked;

impl From<ListenerInput<Pointer<Click>>> for GetMarketClicked {
    fn from(_click_event: ListenerInput<Pointer<Click>>) -> Self { GetMarketClicked }
}

#[derive(Event, Component, Debug)]
pub(crate) struct AutoTradeClicked;

impl From<ListenerInput<Pointer<Click>>> for AutoTradeClicked {
    fn from(_click_event: ListenerInput<Pointer<Click>>) -> Self { AutoTradeClicked }
}

pub(crate) fn handle_orbit_clicked_event(
    selected_ship: Query<&SelectedShip>, mut error_text: Query<&mut Text, With<ErrorText>>,
    mut ships: Query<(Entity, &mut Ship)>, mut ship_selected_event: EventWriter<ShipSelected>,
) {
    if let Ok(selected_ship) = selected_ship.get_single() {
        let res = st_client::orbit_ship(selected_ship.ship.symbol.as_str());
        match serde_json::from_str::<GenericResponse<NavWrapper>>(&res.unwrap()) {
            Ok(nav_details) => {
                for (ship_entity, mut ship) in ships.iter_mut() {
                    if ship.symbol == selected_ship.ship.symbol {
                        ship.nav = nav_details.data.nav.clone();
                        ship_selected_event.send(ShipSelected(ship_entity));
                    }
                }
            }
            Err(e) => {
                panic!("Error reading navigation data when orbiting ship: {e}");
            }
        }
    } else {
        error_text.single_mut().sections[0].value = "Error: No Ship Selected".to_string();
    }
}

pub(crate) fn handle_dock_clicked_event(
    selected_ship: Query<&SelectedShip>, mut error_text: Query<&mut Text, With<ErrorText>>,
    mut ships: Query<(Entity, &mut Ship)>, mut ship_selected_event: EventWriter<ShipSelected>,
) {
    if let Ok(selected_ship) = selected_ship.get_single() {
        let res = st_client::dock_ship(selected_ship.ship.symbol.as_str());
        match serde_json::from_str::<GenericResponse<NavWrapper>>(&res.unwrap()) {
            Ok(nav_details) => {
                for (ship_entity, mut ship) in ships.iter_mut() {
                    if ship.symbol == selected_ship.ship.symbol {
                        ship.nav = nav_details.data.nav.clone();
                        ship_selected_event.send(ShipSelected(ship_entity));
                        break;
                    }
                }
            }
            Err(e) => {
                panic!("Error reading navigation data when docking ship: {e}");
            }
        }
    } else {
        error_text.single_mut().sections[0].value = "Error: No Ship Selected".to_string();
    }
}

pub(crate) fn handle_move_ship(
    selected_ship_query: Query<&SelectedShip>, selected_waypoint: Query<&SelectedWaypoint>,
    mut error_text: Query<&mut Text, With<ErrorText>>, mut ships: Query<(Entity, &mut Ship)>,
    mut ship_selected_event: EventWriter<ShipSelected>,
) {
    if let (Ok(waypoint), Ok(selected_ship)) = (selected_waypoint.get_single(), selected_ship_query.get_single()) {
        let nav = st_client::move_ship(&selected_ship.ship, waypoint.waypoint.symbol.to_string());
        match nav {
            Ok(nav) => {
                for (ship_entity, mut ship) in ships.iter_mut() {
                    if ship.symbol == selected_ship.ship.symbol {
                        ship.nav = nav.clone();
                        ship_selected_event.send(ShipSelected(ship_entity));
                        break;
                    }
                }
            }
            Err(e) => {
                error_text.single_mut().sections[0].value = format!("Error: Unable to move ship: {e}").to_string();
            }
        }
    } else {
        error_text.single_mut().sections[0].value = "Error: You must select a ship and a waypoint".to_string();
    }
}

pub(crate) fn handle_get_market_clicked(
    mut commands: Commands, selected_ship_query: Query<&SelectedShip>, mut error_text: Query<&mut Text, With<ErrorText>>,
    waypoint_query: Query<&Waypoint>, existing_markets_query: Query<(Entity, &Market), With<Market>>,
) {
    if let Ok(selected_ship) = selected_ship_query.get_single() {
        if selected_ship.ship.nav.status == FlightStatus::IN_TRANSIT {
            error_text.single_mut().sections[0].value = "Error: Ship must not be in transit to get market data".to_string();
            return;
        }

        let found_waypoints = waypoint_query
            .iter()
            .filter(|w| w.symbol == selected_ship.ship.nav.waypoint_symbol)
            .collect::<Vec<&Waypoint>>();
        let found_waypoint = found_waypoints.get(0).unwrap();

        if !found_waypoint.has_marketplace() {
            error_text.single_mut().sections[0].value = "Error: Waypoint has no marketplace".to_string();
            return;
        }

        let market_details = st_client::get_market_data(&found_waypoint.system_symbol, &found_waypoint.symbol);
        match market_details {
            Ok(market_data) => {
                for (existing_market_entity, existing_market) in existing_markets_query.iter() {
                    if existing_market.symbol == market_data.symbol {
                        commands.entity(existing_market_entity).despawn_recursive();
                        break;
                    }
                }
                println!("market data: {}", serde_json::to_string_pretty(&market_data).unwrap());
                commands.spawn(market_data);
            }
            Err(e) => {
                println!("{e}");
                error_text.single_mut().sections[0].value = format!("Error: Unable to read market data {e}.").to_string();
            }
        }
    } else {
        error_text.single_mut().sections[0].value = "Error: You must select a ship.".to_string();
    }
}
// pub(crate) fn handle_autotrade_clicked<T: std::marker::Send + std::marker::Sync + 'static>(
pub(crate) fn handle_autotrade_clicked(
    selected_ship_query: Query<&SelectedShip>, mut error_text: Query<&mut Text, With<ErrorText>>, mut ships: Query<(Entity, &Ship, &mut ShipState)>,
) {
    if let Ok(selected_ship) = selected_ship_query.get_single() {

        for (_, ship, mut state) in ships.iter_mut() {
            if ship.symbol == selected_ship.ship.symbol {
                if state.is_idle() {
                    state.state = ShipStateEnum::Autotrade;
                } else {
                    state.state = ShipStateEnum::Idle;
                }
                dbg!(&state);
                
                break;
            }
        }
    } else {
        error_text.single_mut().sections[0].value = "Error: You must select a ship.".to_string();
    }
}

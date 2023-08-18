use bevy::prelude::*;
use bevy::text::Text;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::Click;
use bevy_mod_picking::prelude::Pointer;
use serde::Deserialize;
// use bevy::prelude::Eve

use crate::game::ship::components::Nav;
use crate::game::ship::components::Ship;
use crate::game::ship::systems::events::ShipSelected;
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

#[derive(Deserialize, Clone, Debug)]
struct NavWrapper {
    nav: Nav,
}

pub(crate) fn handle_orbit_clicked_event(
    selected_ship: Query<&SelectedShip>, mut error_text: Query<&mut Text, With<ErrorText>>, mut ships: Query<(Entity, &mut Ship)>, mut ship_selected_event: EventWriter<ShipSelected>
) {
    if let Ok(selected_ship) = selected_ship.get_single() {
        let res = st_client::orbit_ship(selected_ship.ship.symbol.as_str());
        match serde_json::from_str::<GenericResponse<NavWrapper>>(&res) {
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
        match serde_json::from_str::<GenericResponse<NavWrapper>>(&res) {
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
    selected_ship_query: Query<&SelectedShip>, selected_waypoint: Query<&SelectedWaypoint>, mut error_text: Query<&mut Text, With<ErrorText>>,
    mut ships: Query<(Entity, &mut Ship)>, mut ship_selected_event: EventWriter<ShipSelected>,
) {
    if let (Ok(waypoint), Ok(selected_ship)) = (selected_waypoint.get_single(), selected_ship_query.get_single()) {
        let res = st_client::move_ship(selected_ship.ship.symbol.as_str(), waypoint.waypoint.symbol.to_string());
        match serde_json::from_str::<GenericResponse<NavWrapper>>(&res) {
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
                panic!("Error reading navigation data when moving ship: {e}");
            }
        }
    } else {
        error_text.single_mut().sections[0].value = "Error: You must select a ship and a waypoint".to_string();
    }
}


use bevy::prelude::*;
use bevy::text::Text;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::Click;
use bevy_mod_picking::prelude::Pointer;
use serde::Deserialize;

use crate::game::ship::components::Nav;
use crate::game::ship::components::Ship;
use crate::game::waypoint::components::Waypoint;
use crate::st_client;
use crate::st_client::GenericResponse;
use crate::ui::controls::components::SelectedShip;
use crate::ui::controls::components::SelectedWaypointText;
use crate::ui::hud::components::ErrorText;

// finally was able to use events with bevy 0.11 and patched bevy_mod_picking|_events
// see here for an example: https://github.com/chriamue/flyconomy/blob/53d5b1d91aa39b6b97ff348f22ec35e0f1152a1f/src/game/aerodrome.rs#L33
#[derive(Event, Component, Debug)]
pub(crate) struct OrbitClicked;

impl From<ListenerInput<Pointer<Click>>> for OrbitClicked {
    fn from(_click_event: ListenerInput<Pointer<Click>>) -> Self { OrbitClicked }
}

#[derive(Deserialize, Clone, Debug)]
struct NavWrapper {
    nav: Nav
}

pub(crate) fn handle_orbit_clicked_event(
    selected_ship: Query<&SelectedShip>, mut error_text: Query<&mut Text, With<ErrorText>>, mut ships: Query<&mut Ship>,
) {
    // dbg!("orbit clicked");
    dbg!(&selected_ship.iter().len());
    if let Ok(selected_ship) = selected_ship.get_single() {
        dbg!(&selected_ship);
        let res = st_client::orbit_ship(selected_ship.ship.symbol.as_str());
        // text.sections[0].value = "Orbiting".to_string();
        match serde_json::from_str::<GenericResponse<NavWrapper>>(&res) {
            Ok(nav_details) => {
                for mut ship_entity in &mut ships {
                    if ship_entity.symbol == selected_ship.ship.symbol {
                        ship_entity.nav = nav_details.data.nav.clone();
                    }
                }
            }
            Err(e) => {
                dbg!(&res);
                panic!("Error reading navigation data when orbiting ship: {e}");
            }
        }
        // dbg!(&res);
        // let nav_details: GenericResponse<Nav> = serde_json::from_str(&res).unwrap();
        // dbg!("selected");
    } else {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        // dbg!("none selected");
        error_text.single_mut().sections[0].value = "Error: No Ship Selected".to_string();
    }
}
// pub(crate) fn handle_orbit_clicked_event(
//     mut event: EventReader<OrbitClicked>,
//     // mut clicked_button: Query<&Button>,
// ) {
//     for select_event in event.iter() {
//         dbg!(&event.iter().len());
//         dbg!("clicked_button: {:?}", &select_event);
//         // if let Ok((_entity, waypoint)) = clicked_button.get(select_event.0) {
//         //     // clicked_button.single_mut().sections[0].value = format!("Selected Waypoint: {}, Traits: {}", waypoint.symbol, waypoint.get_traits());
//         // }
//     }
// }

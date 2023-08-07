use bevy::{ecs::query::WorldQuery, prelude::*};

use crate::{
    st_client::{self, GenericResponse},
    ui::{
        controls::{components::{SelectedShip, SelectedWaypoint}, self},
        hud::components::ErrorText,
        hud_buttons::components::*,
    }, game::ship::components::{Nav, Ship},
};

#[allow(clippy::type_complexity)]
pub(crate) fn orbit_clicked(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<OrbitButton>)>,
    mut text_query: Query<&mut Text, Without<ErrorText>>, selected_ship: Query<&SelectedShip>,
    mut error_text: Query<&mut Text, With<ErrorText>>,
) {
    for (interaction, children) in &mut interaction_query {
        dbg!("clic");
        let mut text = text_query.get_mut(children[0]).unwrap();
        if *interaction == Interaction::Pressed {
            if let Ok(selected_ship) = selected_ship.get_single() {
                let res = st_client::orbit_ship(selected_ship.ship.symbol.as_str());
                text.sections[0].value = "Orbiting".to_string();
                dbg!(res);
            } else {
                error_text.single_mut().sections[0].value = "Error: No Ship Selected".to_string();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn move_ship_clicked(
    mut move_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<MoveButton>)>,
    mut text_query: Query<&mut Text, Without<ErrorText>>, mut selected_ship: Query<&SelectedShip>,
    selected_waypoint: Query<&SelectedWaypoint>, mut error_text: Query<&mut Text, With<ErrorText>>,
    mut ships: Query<&mut Ship>
) {
    for (interaction, children) in &mut move_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        if *interaction == Interaction::Pressed {
            // Query::get_mut(&mut self, entity);
            // let mut ship: &mut Ship = selected_ship.get_single_mut().unwrap().ship;
            if let (Ok(waypoint), Ok(ship)) = (selected_waypoint.get_single(), selected_ship.get_single()) {
                // if let (Ok(waypoint), ) = (selected_waypoint.get_single(), ) {
                    text.sections[0].value = "Moving".to_string();
                    let res = st_client::move_ship(ship.ship.symbol.as_str(), waypoint.waypoint.symbol.to_string());
                    let nav_details: GenericResponse<Nav> = serde_json::from_str(&res).unwrap();
                    for mut ship_entity in &mut ships {
                        if ship_entity.symbol  == ship.ship.symbol {
                            ship_entity.nav = nav_details.data.clone();
                        }
                    }
                // ship.ship.nav = nav_details.data;
                dbg!(res);
            } else {
                error_text.single_mut().sections[0].value = "Error: You must select a ship and a waypoint".to_string();
            }
        }
    }
}

use bevy::{ecs::query::WorldQuery, prelude::*};

use crate::{
    st_client,
    ui::{
        controls::components::{SelectedShip, SelectedWaypoint},
        hud_buttons::components::*,
    },
};

#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct OrbitButtonQuery<'a> {
    interaction: &'a Interaction,
    // bg_color: &'a mut BackgroundColor,
    border_color: &'a mut BorderColor,
    children: &'a Children,
    with: With<Button>,
    with_two: With<OrbitButton>,
    // without: Without<MoveButton>,
    with_changed: Changed<Interaction>,
}

pub(crate) fn orbit_clicked(
    mut orbit_query: Query<OrbitButtonQuery>, mut text_query: Query<&mut Text>, selected_ship: Query<&SelectedShip>,
) {
    for mut q in &mut orbit_query {
        let mut text = text_query.get_mut(q.children[0]).unwrap();
        if *q.interaction == Interaction::Pressed {
            text.sections[0].value = "Orbiting".to_string();
            q.border_color.0 = Color::RED;
            if let Ok(selected_ship) = selected_ship.get_single() {
                dbg!("found");
            } else {
                dbg!("error");
            }
            // let res = st_client::orbit_ship(selected_ship.get_single().unwrap().ship.symbol.as_str());
            // dbg!(res);
        }
    }
}


#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct MoveButtonQuery<'a> {
    interaction: &'a Interaction,
    bg_color: &'a mut BackgroundColor,
    border_color: &'a mut BorderColor,
    children: &'a Children,
    with: With<Button>,
    with_two: With<MoveButton>,
    without: Without<OrbitButton>,
    with_changed: Changed<Interaction>,
}
pub(crate) fn move_ship_clicked(
    mut move_query: Query<MoveButtonQuery>, mut text_query: Query<&mut Text>, selected_ship: Query<&SelectedShip>,
    selected_waypoint: Query<&SelectedWaypoint>,
) {
    for mut q in &mut move_query {
        let mut text = text_query.get_mut(q.children[0]).unwrap();
        if *q.interaction == Interaction::Pressed {
            text.sections[0].value = "Press".to_string();
            q.border_color.0 = Color::RED;
            dbg!(selected_waypoint.get_single().unwrap());
            let res = st_client::move_ship(
                selected_ship.get_single().unwrap().ship.symbol.as_str(),
                selected_waypoint.get_single().unwrap().waypoint.symbol.to_string(),
            );
            dbg!(res);
        }
    }
}

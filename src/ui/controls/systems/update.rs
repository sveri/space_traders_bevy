use bevy::{prelude::*, input::mouse::{MouseWheel, MouseScrollUnit}};

use crate::{st_client::Waypoint, ship::Ship, ui::{controls::components::*, hud::components::SelectedShipText}};


pub(crate) fn player_camera_control(
    mut mouse_wheel_events: EventReader<MouseWheel>, time: Res<Time>, mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let dist = 5.0 * time.delta().as_secs_f32();

    for mut projection in query.iter_mut() {
        for ev in mouse_wheel_events.iter() {
            let mut log_scale = projection.scale.ln();
            match ev.unit {
                MouseScrollUnit::Line => {
                    println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
                    if ev.y > 0.0 {
                        log_scale -= dist;
                    } else {
                        log_scale += dist;
                    }
                }
                MouseScrollUnit::Pixel => {
                    println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                }
            }
            projection.scale = log_scale.exp();
        }
    }
}

pub(crate) fn mouse_click_handler(
    mut commands: Commands, buttons: Res<Input<MouseButton>>, windows: Query<&Window>, ships: Query<&Ship>,
    waypoints: Query<&Waypoint>, mut select_ship_text: Query<&mut Text, (With<SelectedShipText>, Without<SelectedWaypointText>)>,
    mut select_waypoint_text: Query<&mut Text, (With<SelectedWaypointText>, Without<SelectedShipText>)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = camera_q.single();

        if let Some(world_position) =
            window.cursor_position().and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            if let Some(found_ship) = ships.iter().find(|ship| ship.in_bounds(world_position.x, world_position.y)) {
                select_ship_text.single_mut().sections[0].value = format!("Selected Ship: {}, Status: {}", found_ship.symbol, found_ship.nav.status);
                commands.spawn(SelectedShip {
                    ship: found_ship.clone(),
                });
            } else if let Some(found_waypoint) =
                waypoints.iter().find(|waypoint| waypoint.in_bounds(world_position.x, world_position.y))
            {
                select_waypoint_text.single_mut().sections[0].value =
                    format!("Selected Waypoint: {} - {}", found_waypoint.symbol, found_waypoint.get_traits());
                commands.spawn(SelectedWaypoint { waypoint: found_waypoint.clone()});
            } else {
                select_ship_text.single_mut().sections[0].value = "Selected Ship: ".to_string();
                select_waypoint_text.single_mut().sections[0].value = "Selected Waypoint: ".to_string();
            }
        }
    }
}
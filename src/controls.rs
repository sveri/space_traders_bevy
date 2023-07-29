use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::{
        Camera, Commands, Component, EventReader, GlobalTransform, Input, MouseButton, OrthographicProjection, Query, Res, With,
        Without,
    },
    text::Text,
    time::Time,
    transform::commands,
    window::Window,
};

use crate::{ship::Ship, st_client::Waypoint};

/// Used to help identify our main camera
#[derive(Component)]
pub(crate) struct MainCamera;

#[derive(Component)]
pub(crate) struct SelectedWaypointText;

#[derive(Component)]
pub(crate) struct SelectedShipText;

#[derive(Component, Debug)]
pub(crate) struct SelectedShip {
    pub(crate) ship: Ship,
}

#[derive(Component, Debug)]
pub(crate) struct SelectedWaypoint {
    pub(crate)  waypoint: Waypoint,
}

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

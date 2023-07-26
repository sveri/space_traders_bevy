use std::error::Error;

use bevy::prelude::*;

// use crate::ui;

mod controls;
mod ship;
mod st_client;
mod ui;
mod util;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct ShipComponent;

#[derive(Component)]
struct WaypointComponent;

fn add_ships(mut commands: Commands) {
    let ships = ship::fetch_my_ships();
    ships.iter().for_each(|s| {
        commands.spawn((s.to_owned(), ShipComponent));
    })
}

fn add_waypoints(mut commands: Commands) {
    let agent_details = st_client::fetch_agent_details();
    let waypoints = st_client::fetch_waypoints(agent_details.get_headquarter_system_symbol().as_str());
    waypoints.iter().for_each(|w| {
        commands.spawn((w.to_owned(), WaypointComponent));
    })
}

fn setup(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.projection.scale = 0.234;
    commands.spawn((bundle, controls::MainCamera));
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ui::PlanetUpdateTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(ui::ShipUpdateTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(
                Startup,
                (setup, add_ships, add_waypoints, ui::get_agent_details, ui::selected_ship_text, ui::selected_waypoint_text),
            )
            .add_systems(Update, (ui::show_waypoints, ui::show_ships, controls::player_camera_control, controls::mouse_click_handler));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    App::new().add_plugins((DefaultPlugins, MainPlugin)).run();

    Ok(())
}

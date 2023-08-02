use std::error::Error;

use bevy::prelude::*;
use game::ship;

// mod ship;
mod st_client;
mod ui;
mod util;
mod game;


#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct WaypointComponent;

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
    commands.spawn((bundle, crate::ui::controls::components::MainCamera));
}

struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((game::GamePlugin, ui::UiPlugin))

            .add_systems(Startup, (setup, add_waypoints));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    App::new()
        .add_plugins((DefaultPlugins, MainPlugin))
        .add_plugins(bevy_framepace::FramepacePlugin)
        // .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin, bevy_framepace::FramepacePlugin))
        .run();

    Ok(())
}

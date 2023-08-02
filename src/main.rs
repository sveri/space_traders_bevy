mod game;
mod st_client;
mod ui;
mod util;

use std::error::Error;

use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn setup_camera(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.projection.scale = 0.234;
    commands.spawn((bundle, crate::ui::controls::components::MainCamera));
}

struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) { app.add_plugins((game::GamePlugin, ui::UiPlugin)).add_systems(Startup, setup_camera); }
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

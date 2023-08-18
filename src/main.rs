mod game;
mod st_client;
mod ui;

use std::error::Error;

use bevy::{prelude::*, winit::WinitSettings, log::LogPlugin, window::PresentMode};
use bevy_mod_picking::{DefaultPickingPlugins, prelude::RaycastPickCamera};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn setup_camera(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.projection.scale = 0.134;
    commands.spawn((bundle, crate::ui::controls::components::MainCamera, RaycastPickCamera::default(), ));
}

struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) { app.add_plugins((game::GamePlugin, ui::UiPlugin)).add_systems(Startup, setup_camera); }
}

fn main() -> Result<(), Box<dyn Error>> {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(_) => {panic!("please proved a .env file with the space traders key")}
    }

    App::new()
        // .insert_resource(WinitSettings::desktop_app())
        
        .add_plugins((DefaultPlugins.set(LogPlugin {
            filter: "warn,mygame=debug".into(),
            level: bevy::log::Level::DEBUG,
        }).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Traders".into(),
                resolution: (1900., 1100.).into(),
                position: WindowPosition::At((300, 80).into()),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }), MainPlugin))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(DefaultPickingPlugins)
        // .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin, bevy_framepace::FramepacePlugin))
        .run();

    Ok(())
}

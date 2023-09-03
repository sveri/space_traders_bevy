mod game;
mod st_client;
mod ui;

use std::{error::Error, time::Duration};

use bevy::{
    app::PluginGroupBuilder, log::LogPlugin, prelude::*, time::common_conditions::on_timer, window::PresentMode,
    winit::WinitSettings,
};
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{prelude::RaycastPickCamera, DefaultPickingPlugins};
use bevy_save::{AppSaveableExt, SavePlugins, WorldSaveableExt};
use game::{
    components::{ImportExport, Market, TradeGood, Transaction},
    ship::components::Ship,
};
use tracing_subscriber::EnvFilter;

#[derive(Component)]
struct Person;

fn setup_camera(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.projection.scale = 0.134;
    commands.spawn((
        bundle,
        crate::ui::controls::components::MainCamera,
        RaycastPickCamera::default(),        
        Name::new("Main Camera".to_string()),
    )).insert(PanCam::default());
}

fn save_world(world: &World) { world.save("space_traders").expect("Failed to save"); }

fn load_world(world: &mut World) {
    match world.load("space_traders") {
        Ok(_) => {
            tracing::info!("Loaded savefile");
        }
        Err(e) => {
            tracing::error!("Failed to load save, probably because it doesn't exist yet");
            tracing::error!("{e}");
        }
    }
}

struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SavePlugins)
            .register_saveable::<Market>()
            .register_saveable::<ImportExport>()
            .register_saveable::<Vec<ImportExport>>()
            .register_saveable::<Transaction>()
            .register_saveable::<Vec<Transaction>>()
            .register_saveable::<TradeGood>()
            .register_saveable::<Vec<TradeGood>>()
            .add_systems(Startup, (load_world))
            .add_systems(Update, save_world.run_if(on_timer(Duration::from_secs(10))));
    }
}


struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((game::GamePlugin, ui::UiPlugin, SavePlugin))
            // .add_plugins(PanOrbitCameraPlugin)
            .add_plugins( PanCamPlugin::default())
            .add_systems(Startup, setup_camera);
    }
}

struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new())
        .register_type::<Ship>()
        // .register_type::<ShipState>()
        ;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(_) => {
            panic!("please proved a .env file with the space traders key")
        }
    }

    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    // tracing::subscriber::set_global_default(subscriber)?;

    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        // .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Starting up");

    App::new()
        // .insert_resource(WinitSettings::desktop_app())
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    filter: "warn,mygame=debug".into(),
                    level: bevy::log::Level::DEBUG,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Space Traders".into(),
                        resolution: (1900., 1100.).into(),
                        position: WindowPosition::At((300, 80).into()),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            MainPlugin,
            DebugPlugin,
        ))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(DefaultPickingPlugins)
        // .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin, bevy_framepace::FramepacePlugin))
        .run();

    Ok(())
}

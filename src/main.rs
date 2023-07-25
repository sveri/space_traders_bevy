use std::error::Error;

use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle},
};

use bevy_mod_reqwest::*;
use ship::Ship;
use st_client::Waypoint;

mod ship;
mod st_client;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct PlanetUpdateTimer(Timer);

#[derive(Resource)]
struct ShipUpdateTimer(Timer);

fn add_ships(mut commands: Commands) {
    let ships = ship::fetch_my_ships();
    ships.iter().for_each(|s| {
        commands.spawn((s.to_owned()));
    })
}

fn add_waypoints(mut commands: Commands) {
    let agent_details = st_client::fetch_agent_details();
    let waypoints = st_client::fetch_waypoints(agent_details.get_headquarter_system_symbol().as_str());
    waypoints.iter().for_each(|w| {
        commands.spawn((w.to_owned()));
    })
}

fn setup(mut commands: Commands) { commands.spawn(Camera2dBundle::default()); }

#[derive(Component)]
struct AgentDetailsText;

fn get_agent_details(mut commands: Commands) {
    let agent_details = st_client::fetch_agent_details();

    commands.spawn((TextBundle::from_section(
        format!(
            "{}, Faction: {} HQs: {}, HQ system symbol: {}",
            agent_details.symbol,
            agent_details.starting_faction,
            agent_details.headquarters,
            agent_details.get_headquarter_system_symbol()
        ),
        TextStyle {
            font_size: 15.0,
            color: Color::WHITE,
            ..default()
        },
    )
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(0.0),
        left: Val::Px(0.0),
        ..default()
    }),));
}

fn show_ships(time: Res<Time>, mut timer: ResMut<ShipUpdateTimer>, mut commands: Commands, query: Query<&Ship>) {
    if timer.0.tick(time.delta()).just_finished() {
        for ship in query.iter() {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(10.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    ship.nav.route.departure.x as f32,
                    ship.nav.route.departure.y as f32,
                    0.,
                )),
                ..default()
            });
        }
    }
}

fn show_waypoints(
    time: Res<Time>, mut timer: ResMut<PlanetUpdateTimer>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>, query: Query<&Waypoint>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for waypoint in query.iter() {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(waypoint.x as f32, waypoint.y as f32, 0.)),
                ..default()
            });
        }
    }
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlanetUpdateTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(ShipUpdateTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            // .add_plugins(ReqwestPlugin)
            .add_systems(Startup, (setup, add_ships, add_waypoints, get_agent_details))
            .add_systems(Update, (show_waypoints, show_ships));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    App::new().add_plugins((DefaultPlugins, MainPlugin)).run();

    Ok(())
}

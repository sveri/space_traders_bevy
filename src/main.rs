use std::error::Error;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use ship::Ship;
use st_client::Waypoint;

mod ship;
mod st_client;
mod util;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct ShipComponent;

#[derive(Component)]
struct WaypointComponent;

#[derive(Component)]
struct ShipRepresentation;

#[derive(Component)]
struct WaypointRepresentation;

#[derive(Resource)]
struct PlanetUpdateTimer(Timer);

#[derive(Resource)]
struct ShipUpdateTimer(Timer);

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

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.projection.scale = 0.234;
    commands.spawn((bundle, MainCamera));
}

#[derive(Component)]
struct AgentDetailsText;

fn get_agent_details(mut commands: Commands) {
    let agent_details = st_client::fetch_agent_details();

    commands.spawn((
        TextBundle::from_section(
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
        }),
        AgentDetailsText,
    ));
}

fn show_ships(time: Res<Time>, mut timer: ResMut<ShipUpdateTimer>, mut commands: Commands, ships: Query<&Ship>) {
    if timer.0.tick(time.delta()).just_finished() {
        for ship in ships.iter() {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(ship.get_display_size().0, ship.get_display_size().1)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(ship.nav.route.departure.x, ship.nav.route.departure.y, 0.)),
                    ..default()
                },
                ShipRepresentation,
            ));
        }
    }
}

fn show_waypoints(
    time: Res<Time>, mut timer: ResMut<PlanetUpdateTimer>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>, query: Query<&Waypoint>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for waypoint in query.iter() {
            commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(waypoint.get_display_size()).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(waypoint.x as f32, waypoint.y as f32, 0.)),
                ..default()
            }, WaypointRepresentation));
        }
    }
}

fn player_camera_control(
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

fn mouse_click_handler(
    buttons: Res<Input<MouseButton>>, windows: Query<&Window>, ships: Query<&Ship>, waypoints: Query<&Waypoint>,
    mut select_ship_text: Query<&mut Text, (With<SelectedShipText>, Without<SelectedWaypointText>)>, mut select_waypoint_text: Query<&mut Text, (With<SelectedWaypointText>, Without<SelectedShipText>)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = camera_q.single();

        if let Some(world_position) =
            window.cursor_position().and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            if let Some(found_ship) = ships.iter().find(|ship| ship.in_bounds(world_position.x, world_position.y)) {
                select_ship_text.single_mut().sections[0].value = format!("Selected Ship: {}", found_ship.symbol,);
            } else if let Some(found_waypoint) = waypoints.iter().find(|waypoint| waypoint.in_bounds(world_position.x, world_position.y)) {
                select_waypoint_text.single_mut().sections[0].value = format!("Selected Waypoint: {}", found_waypoint.symbol,);
            } else {
                select_ship_text.single_mut().sections[0].value = "Selected Ship: ".to_string();
                select_waypoint_text.single_mut().sections[0].value = "Selected Waypoint: ".to_string();
            }
        }
    }
}

#[derive(Component)]
struct SelectedShipText;
fn selected_ship_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Selected Ship: ",
            TextStyle {
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(0.0),
            ..default()
        }),
        SelectedShipText,
    ));
}

#[derive(Component)]
struct SelectedWaypointText;
fn selected_waypoint_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Selected Waypoint: ",
            TextStyle {
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(0.0),
            ..default()
        }),
        SelectedWaypointText,
    ));
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlanetUpdateTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(ShipUpdateTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(
                Startup,
                (setup, add_ships, add_waypoints, get_agent_details, selected_ship_text, selected_waypoint_text),
            )
            .add_systems(Update, (show_waypoints, show_ships, player_camera_control, mouse_click_handler));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    App::new().add_plugins((DefaultPlugins, MainPlugin)).run();

    Ok(())
}

use std::{error::Error};

use bevy::{
    ecs::query::{WorldQuery},
    prelude::*,
};

// use crate::ui;

mod controls;
mod ship;
mod st_client;
mod ui;
mod util;

mod hud;

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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Component)]
struct MoveButton;

#[derive(Component)]
struct OrbitButton;

fn setup_move_button(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(180.0),
                            height: Val::Px(20.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            top: Val::Px(40.0),
                            right: Val::Px(20.0),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MoveButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Move Ship",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(20.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            top: Val::Px(10.0),
                            right: Val::Px(20.0),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    OrbitButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Orbit",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

#[derive(WorldQuery)]
#[world_query(mutable)]
struct MoveButtonQuery<'a> {
    interaction: &'a Interaction,
    bg_color: &'a mut BackgroundColor,
    border_color: &'a mut BorderColor,
    children: &'a Children,
    with: With<Button>,
    with_two: With<MoveButton>,
    without: Without<OrbitButton>,
    with_changed: Changed<Interaction>,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
struct OrbitButtonQuery<'a> {
    interaction: &'a Interaction,
    bg_color: &'a mut BackgroundColor,
    border_color: &'a mut BorderColor,
    children: &'a Children,
    with: With<Button>,
    with_two: With<OrbitButton>,
    without: Without<MoveButton>,
    with_changed: Changed<Interaction>,
}

fn move_button_system(
    mut move_query: Query<MoveButtonQuery>, mut orbit_query: Query<OrbitButtonQuery>, mut text_query: Query<&mut Text>,
    selected_ship: Query<&controls::SelectedShip>, selected_waypoint: Query<&controls::SelectedWaypoint>,
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

    for mut q in &mut orbit_query {
        let mut text = text_query.get_mut(q.children[0]).unwrap();
        if *q.interaction == Interaction::Pressed {
            text.sections[0].value = "Orbiting".to_string();
            q.border_color.0 = Color::RED;
            let res = st_client::orbit_ship(selected_ship.get_single().unwrap().ship.symbol.as_str());
            dbg!(res);
        }
    }
}

struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ui::UiPlugin)
            .add_systems(Startup, (setup, setup_move_button, add_ships, add_waypoints))
            .add_systems(
                Update,
                (
                    controls::player_camera_control,
                    controls::mouse_click_handler,
                    move_button_system,
                ),
            );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    App::new()
        .add_plugins((DefaultPlugins, MainPlugin))
        .add_plugins(bevy_framepace::FramepacePlugin)
        // .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin, bevy_framepace::FramepacePlugin))
        .run();
    // App::new().add_plugins((DefaultPlugins, MainPlugin, bevy_framepace::FramepacePlugin)).run();

    Ok(())
}

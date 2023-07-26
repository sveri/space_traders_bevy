use bevy::{prelude::*, text::TextStyle, sprite::MaterialMesh2dBundle};

use crate::{st_client::{self, Waypoint}, ship::Ship, controls};



#[derive(Resource)]
pub(crate) struct PlanetUpdateTimer(pub Timer);

#[derive(Resource)]
pub(crate) struct ShipUpdateTimer(pub Timer);

#[derive(Component)]
struct ShipRepresentation;

#[derive(Component)]
struct WaypointRepresentation;

#[derive(Component)]
struct AgentDetailsText;

pub(crate) fn get_agent_details(mut commands: Commands) {
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

pub(crate) fn show_ships(time: Res<Time>, mut timer: ResMut<ShipUpdateTimer>, mut commands: Commands, ships: Query<&Ship>) {
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

pub(crate) fn show_waypoints(
    time: Res<Time>, mut timer: ResMut<PlanetUpdateTimer>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>, query: Query<&Waypoint>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for waypoint in query.iter() {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(waypoint.get_display_size()).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    transform: Transform::from_translation(Vec3::new(waypoint.x, waypoint.y, 0.)),
                    ..default()
                },
                WaypointRepresentation,
            ));
        }
    }
}

pub(crate) fn selected_ship_text(mut commands: Commands) {
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
        controls::SelectedShipText,
    ));
}

pub(crate) fn selected_waypoint_text(mut commands: Commands) {
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
        controls::SelectedWaypointText,
    ));
}
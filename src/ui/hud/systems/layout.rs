use bevy::{prelude::*, text::TextStyle};

use crate::ui::controls::components::SelectedWaypointText;
use crate::st_client;
use crate::ui::hud::components::*;


pub(crate) fn show_agent_details(mut commands: Commands) {
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
        SelectedShipText,
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
        SelectedWaypointText,
    ));
}

pub(crate) fn init_error_text(mut commands: Commands){
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 17.0,
                color: Color::RED,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ErrorText));
}
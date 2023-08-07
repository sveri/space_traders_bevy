use bevy::{prelude::*, text::TextStyle};
use bevy_eventlistener::prelude::On;
use bevy_mod_picking::prelude::{Pointer, Click};

use crate::ui::hud_buttons::components::*;

use super::events::OrbitClicked;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub(in crate::ui::hud_buttons) fn setup_buttons(mut commands: Commands) {
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
                    On::<Pointer<Click>>::send_event::<OrbitClicked>(),
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

use bevy::{prelude::*, text::TextStyle};
use bevy_eventlistener::prelude::On;
use bevy_mod_picking::prelude::{Click, Pointer};

use crate::ui::hud_buttons::components::*;

use super::events::{OrbitClicked, DockClicked};

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
                    button_bundle(10., 20.),
                    OrbitButton,
                    On::<Pointer<Click>>::send_event::<DockClicked>(),
                ))
                .with_children(|parent| {
                    parent.spawn(text_bundle("Dock Ship"));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    button_bundle(40., 20.),
                    OrbitButton,
                    On::<Pointer<Click>>::send_event::<OrbitClicked>(),
                ))
                .with_children(|parent| {
                    parent.spawn(text_bundle("Orbit Ship"));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    button_bundle(70., 20.),
                    MoveButton,
                ))
                .with_children(|parent| {
                    parent.spawn(text_bundle("Move Ship"));
                });
        });
}

fn button_bundle(top: f32, right: f32) -> ButtonBundle {
    ButtonBundle {
        style: menu_button_style(top, right),
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

fn menu_button_style(top: f32, right: f32) -> Style {
    Style {
        top: Val::Px(top),
        right: Val::Px(right),
        width: Val::Px(130.0),
        height: Val::Px(20.0),
        border: UiRect::all(Val::Px(5.0)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        position_type: PositionType::Absolute,
        ..default()
    }
}

fn text_bundle(text: &str) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size: 20.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    )
}
use bevy::{prelude::*, ui::FocusPolicy};

use crate::game::market::components::MarketMarker;

pub(crate) fn init_market_overview(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                focus_policy: FocusPolicy::Block,
                visibility: Visibility::Visible,
                background_color: BackgroundColor(Color::BLACK),
                z_index: ZIndex::Global(10),
                ..default()
            },
            MarketMarker,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Use the panel on the right to change the Display and Visibility properties for the respective nodes of the \
                     panel on the left",
                    TextStyle {
                        font_size: 25.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..Default::default()
            });
        });
}

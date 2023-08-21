use bevy::prelude::*;

pub(crate) fn init_market_overview(mut commands: Commands) {
    dbg!("show_market_overview");
    commands.spawn(
        NodeBundle {
            visibility: Visibility::Visible,
            ..default()
        }
        // TextBundle::from_section(
        //     "öaslkdf",
        //     TextStyle {
        //         font_size: 15.0,
        //         color: Color::WHITE,
        //         ..default()
        //     },
        // )
        // .with_text_alignment(TextAlignment::Center)
        // .with_style(Style {
        //     position_type: PositionType::Absolute,
        //     top: Val::Px(600.0),
        //     left: Val::Px(0.0),
        //     ..default()
        // }),
    );
}

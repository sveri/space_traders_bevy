use bevy::prelude::*;

use super::components::MarketMarker;


pub(crate) fn show_hide_market(mut commands: Commands, mut node_bundle_query: Query<(&mut Visibility, &ComputedVisibility), With<MarketMarker>>) {
    
    let (mut visibility, computed_visibility)= node_bundle_query.get_single_mut().unwrap();
    if computed_visibility.is_visible() {
        *visibility = Visibility::Hidden;
    } else {
        *visibility = Visibility::Visible;
    }
    

    // commands.spawn(
    //     NodeBundle {
    //         visibility: Visibility::Visible,
    //         ..default()
    //     }
    //     // TextBundle::from_section(
    //     //     "öaslkdf",
    //     //     TextStyle {
    //     //         font_size: 15.0,
    //     //         color: Color::WHITE,
    //     //         ..default()
    //     //     },
    //     // )
    //     // .with_text_alignment(TextAlignment::Center)
    //     // .with_style(Style {
    //     //     position_type: PositionType::Absolute,
    //     //     top: Val::Px(600.0),
    //     //     left: Val::Px(0.0),
    //     //     ..default()
    //     // }),
    // );
}

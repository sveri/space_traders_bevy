use bevy::{prelude::*, ui::FocusPolicy};

use crate::game::components::Market;

use super::components::MarketMarker;

pub(crate) fn show_hide_market(
    // mut commands: Commands, mut node_bundle_query: Query<(&mut Visibility, &ComputedVisibility), With<MarketMarker>>,
    mut commands: Commands, mut node_bundle_query: Query<&ComputedVisibility, With<MarketMarker>>, market_entity: Query<Entity, With<MarketMarker>>,
    markets_query: Query<&Market>,
) {
    // let (mut visibility, computed_visibility)= node_bundle_query.get_single_mut().unwrap();
    // if computed_visibility.is_visible() {
    //     *visibility = Visibility::Hidden;
    // } else {
    //     *visibility = Visibility::Visible;
    // }
    
    // let new_visibility =  match node_bundle_query.get_single_mut() {
    //     Ok(x) => if x.is_visible() {
    //         Visibility::Hidden
    //     } else {
    //         Visibility::Visible
    //     },
    //     Err(_) => Visibility::Visible,
    // };
    if market_entity.get_single().is_ok() {
        commands.entity(market_entity.get_single().unwrap()).despawn_recursive();
        return;
    }

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                focus_policy: FocusPolicy::Block,
                // visibility: new_visibility,
                visibility: Visibility::Visible,
                background_color: BackgroundColor(Color::BLACK),
                z_index: ZIndex::Global(10),

                ..default()
            },
            MarketMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Stretch,
                        width: Val::Percent(100.),
                        // height: Val::Px(100.),
                        padding: UiRect::all(Val::Px(4.)),
                        ..Default::default()
                    },
                    // style: Style {
                    //     flex_direction: FlexDirection::Column,
                    //     align_items: AlignItems::Center,
                    //     justify_content: JustifyContent::Center,
                    //     ..default()
                    // },
                    // material: materials::MATERIAL_WHITE.clone(),
                    ..default()
                })
                .with_children(|parent| {
                    for market in markets_query.iter() {
                        dbg!("sdlkfjdsf");

                        parent.spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Stretch,
                                width: Val::Percent(100.),
                                justify_content: JustifyContent::Center,
                                // height: Val::Px(100.),
                                padding: UiRect::all(Val::Px(4.)),
                                ..Default::default()
                            },
                            ..default()
                        }).with_children(|parent| {

                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    market.symbol.clone(),
                                    TextStyle {
                                        font_size: 25.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                                ..Default::default()
                            });
                        });

                    }
                });
        });
}

// pub(crate) fn show_hide_market(mut commands: Commands, mut node_bundle_query: Query<(&mut Visibility, &ComputedVisibility), With<MarketMarker>>, markets_query: Query<&Market>) {

//     let (mut visibility, computed_visibility)= node_bundle_query.get_single_mut().unwrap();
//     if computed_visibility.is_visible() {
//         *visibility = Visibility::Hidden;
//     } else {
//         *visibility = Visibility::Visible;
//     }

//     // for market in markets_query.iter() {
//     //     dbg!(&market.symbol);
//     // }
// }

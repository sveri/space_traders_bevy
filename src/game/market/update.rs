use bevy::{prelude::*, ui::FocusPolicy};
use serde::de;

use crate::game::components::Market;

use super::components::MarketMarker;

pub(crate) fn show_hide_market(
    // mut commands: Commands, mut node_bundle_query: Query<(&mut Visibility, &ComputedVisibility), With<MarketMarker>>,
    mut commands: Commands,
    mut node_bundle_query: Query<&ComputedVisibility, With<MarketMarker>>,
    market_entity: Query<Entity, With<MarketMarker>>,
    markets_query: Query<&Market>,
) {
    if market_entity.get_single().is_ok() {
        commands.entity(market_entity.get_single().unwrap()).despawn_recursive();
        return;
    }

    let column_headlines = ["Market", "Exports", "Imports", "tradeGoods"];

    let text_style = TextStyle {
        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    };
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                }).with_children(|parent| {


                    parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                });

                    parent.spawn(TextBundle {
                        text: Text::from_section("123".to_string(), text_style.clone()),
                        ..Default::default()
                    });
                    parent.spawn(TextBundle {
                        text: Text::from_section("123".to_string(), text_style.clone()),
                        ..Default::default()
                    });
                });

});

    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             position_type: PositionType::Absolute,
    //             width: Val::Percent(100.),
    //             height: Val::Percent(100.),
    //             justify_content: JustifyContent::Center,
    //             align_items: AlignItems::Center,
    //             ..Default::default()
    //         },
    //         background_color: Color::BLACK.into(),
    //         ..Default::default()
    //     })
    //     .with_children(|parent| {
    //         parent
    //             .spawn(NodeBundle {
    //                 style: Style {
    //                     flex_direction: FlexDirection::Column,
    //                     align_items: AlignItems::Center,
    //                     justify_content: JustifyContent::Center,
    //                     ..Default::default()
    //                 },
    //                 ..Default::default()
    //             })
    //             .with_children(|parent| {

    //                 parent
    //                     .spawn(NodeBundle {
    //                         style: Style {
    //                             flex_direction: FlexDirection::Column,
    //                             align_items: AlignItems::Stretch,
    //                             padding: UiRect::all(Val::Px(10.)),
    //                             margin: UiRect::top(Val::Px(50.)),
    //                             ..Default::default()
    //                         },
    //                         background_color: Color::YELLOW.into(),
    //                         ..Default::default()
    //                     })
    //                     .with_children(|parent| {
    //                         for title in column_headlines {
    //                             spawn_headline_row(parent, title, text_style.clone());
    //                         }
    //                         // for title in column_headlines {
    //                         //     spawn_button_row(parent, constaint, text_style.clone(), &markets_query);
    //                         // }
    //                     });

    //                 parent.spawn(TextBundle::from_section("Size Constraints Example", text_style.clone()).with_style(Style {
    //                     margin: UiRect::bottom(Val::Px(25.)),
    //                     ..Default::default()
    //                 }));
    //                 parent.spawn(TextBundle::from_section("Size Constraints Example", text_style.clone()).with_style(Style {
    //                     margin: UiRect::bottom(Val::Px(25.)),
    //                     ..Default::default()
    //                 }));

    //                 // spawn_bar(parent);
    //             });
    //     });
}


fn spawn_headline_row(parent: &mut ChildBuilder, title: &str, text_style: TextStyle) {
    let label = "label".to_string();

    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(2.)),
                align_items: AlignItems::Stretch,
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        padding: UiRect::all(Val::Px(2.)),
                        ..Default::default()
                    },
                    //background_color: Color::RED.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // spawn row label
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                min_width: Val::Px(200.),
                                max_width: Val::Px(200.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(title.to_string(), text_style.clone()),
                                ..Default::default()
                            });
                        });

                    // spawn row buttons
                    parent
                        .spawn(NodeBundle {
                            // background_color: Color::DARK_GREEN.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // for market in markets_query.iter() {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(
                                        // market.symbol.clone(),
                                        "sldkjf".to_string(),
                                        text_style.clone(),
                                    ),
                                    ..Default::default()
                                });
                            // }
                        });
                });
        });
}

fn spawn_button_row(parent: &mut ChildBuilder, constraint: &str, text_style: TextStyle, markets_query: &Query<&Market>) {
    let label = "label".to_string();

    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(2.)),
                align_items: AlignItems::Stretch,
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        padding: UiRect::all(Val::Px(2.)),
                        ..Default::default()
                    },
                    //background_color: Color::RED.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // spawn row label
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                min_width: Val::Px(200.),
                                max_width: Val::Px(200.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(label.to_string(), text_style.clone()),
                                ..Default::default()
                            });
                        });

                    // spawn row buttons
                    parent
                        .spawn(NodeBundle {
                            // background_color: Color::DARK_GREEN.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            for market in markets_query.iter() {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(
                                        market.symbol.clone(),
                                        // "sldkjf".to_string(),
                                        TextStyle {
                                            font_size: 25.0,
                                            color: Color::WHITE,
                                            ..default()
                                        },
                                    ),
                                    ..Default::default()
                                });
                            }
                        });
                });
        });
}

// commands
//     .spawn((
//         NodeBundle {
//             style: Style {
//                 position_type: PositionType::Absolute,
//                 width: Val::Percent(100.),
//                 height: Val::Percent(100.),
//                 top: Val::Px(0.0),
//                 left: Val::Px(0.0),
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 ..default()
//             },
//             focus_policy: FocusPolicy::Block,
//             // visibility: new_visibility,
//             visibility: Visibility::Visible,
//             background_color: BackgroundColor(Color::BLACK),
//             z_index: ZIndex::Global(10),

//             ..default()
//         },
//         MarketMarker,
//     ))
//     .with_children(|parent| {
//         parent
//             .spawn(NodeBundle {
//                 style: Style {
//                     align_items: AlignItems::Stretch,
//                     width: Val::Percent(100.),
//                     // height: Val::Px(100.),
//                     padding: UiRect::all(Val::Px(4.)),
//                     ..Default::default()
//                 },
//                 // style: Style {
//                 //     flex_direction: FlexDirection::Column,
//                 //     align_items: AlignItems::Center,
//                 //     justify_content: JustifyContent::Center,
//                 //     ..default()
//                 // },
//                 // material: materials::MATERIAL_WHITE.clone(),
//                 ..default()
//             })
//             .with_children(|parent| {
//                 for market in markets_query.iter() {
//                     dbg!("sdlkfjdsf");

//                     parent.spawn(NodeBundle {
//                         style: Style {
//                             align_items: AlignItems::Stretch,
//                             width: Val::Percent(100.),
//                             justify_content: JustifyContent::Center,
//                             // height: Val::Px(100.),
//                             padding: UiRect::all(Val::Px(4.)),
//                             ..Default::default()
//                         },
//                         ..default()
//                     }).with_children(|parent| {

//                         parent.spawn(TextBundle {
//                             text: Text::from_section(
//                                 market.symbol.clone(),
//                                 TextStyle {
//                                     font_size: 25.0,
//                                     color: Color::WHITE,
//                                     ..default()
//                                 },
//                             ),
//                             ..Default::default()
//                         });
//                     });

//                 }
//             });
//     });
// }

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

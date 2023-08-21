use bevy::prelude::*;

use crate::game::market::components::MarketMarker;

pub(crate) fn init_market_overview(mut commands: Commands) {
    dbg!("show_market_overview");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(600.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                visibility: Visibility::Visible,
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
    // commands.spawn(NodeBundle {
    //     style: Style {
    //         flex_direction: FlexDirection::Column,
    //         flex_basis: Val::Percent(100.),
    //         align_items: AlignItems::Center,
    //         justify_content: JustifyContent::SpaceEvenly,
    //         ..Default::default()
    //     },
    //     background_color: BackgroundColor(Color::BLACK),
    //     ..Default::default()
    // }).with_children(|parent| {
    //     parent.spawn(TextBundle {
    //         text: Text::from_section(
    //             "Use the panel on the right to change the Display and Visibility properties for the respective nodes of the panel on the left",
    //             text_style.clone(),
    //         ).with_alignment(TextAlignment::Center),
    //         style: Style {
    //             margin: UiRect::bottom(Val::Px(10.)),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     });

    //     parent
    //         .spawn(NodeBundle {
    //             style: Style {
    //                 width: Val::Percent(100.),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         })
    //         .with_children(|parent| {
    //             let mut target_ids = vec![];
    //             parent.spawn(NodeBundle {
    //                 style: Style {
    //                     width: Val::Percent(50.),
    //                     height: Val::Px(520.),
    //                     justify_content: JustifyContent::Center,
    //                     ..Default::default()
    //                 },
    //                 ..Default::default()
    //             }).with_children(|parent| {
    //                 target_ids = spawn_left_panel(parent, &palette);
    //             });

    //             parent.spawn(NodeBundle {
    //                 style: Style {
    //                     width: Val::Percent(50.),
    //                     justify_content: JustifyContent::Center,
    //                     ..Default::default()
    //                 },
    //                 ..Default::default()
    //             }).with_children(|parent| {
    //                 spawn_right_panel(parent, text_style, &palette, target_ids);
    //             });
    //         });

    //         parent.spawn(NodeBundle {
    //             style: Style {
    //                 flex_direction: FlexDirection::Row,
    //                 align_items: AlignItems::Start,
    //                 justify_content: JustifyContent::Start,
    //                 column_gap: Val::Px(10.),
    //                 ..Default::default()
    //             },
    //             ..default() })
    //         .with_children(|builder| {
    //             let text_style = TextStyle {
    //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                 font_size: 20.0,
    //                 color: Color::WHITE,
    //             };

    //             builder.spawn(TextBundle {
    //                 text: Text::from_section(
    //                     "Display::None\nVisibility::Hidden\nVisbility::Inherited",
    //                     TextStyle { color: HIDDEN_COLOR, ..text_style.clone() }
    //                     ).with_alignment(TextAlignment::Center),
    //                 ..Default::default()
    //                 });
    //                 builder.spawn(TextBundle {
    //                     text: Text::from_section(
    //                         "-\n-\n-",
    //                         TextStyle { color: Color::DARK_GRAY, ..text_style.clone() }
    //                         ).with_alignment(TextAlignment::Center),
    //                     ..Default::default()
    //                     });
    //                 builder.spawn(TextBundle::from_section(
    //                     "The UI Node and its descendants will not be visible and will not be alloted any space in the UI layout.\nThe UI Node will not be visible but will still occupy space in the UI layout.\nThe UI node will inherit the visibility property of its parent. If it has no parent it will be visible.",
    //                     text_style
    //                 ));
    //         });
    // });
}

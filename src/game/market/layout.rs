use bevy::{prelude::*, ui::FocusPolicy};

use crate::game::{components::Market, market::components::MarketMarker};

pub(crate) fn init_market_overview(mut commands: Commands, markets_query: Query<&Market>) {
    let mut market_symbols = vec![];
    for market in markets_query.iter() {
        market_symbols.push(market.symbol.clone());

        dbg!(market);
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
                visibility: Visibility::Hidden,
                background_color: BackgroundColor(Color::BLACK),
                z_index: ZIndex::Global(10),

                ..default()
            },
            MarketMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
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
                    }
                });
            // parent.spawn(TextBundle {
            //     text: Text::from_section(
            //         "Use the panel on the right to change the Display and Visibility properties for the respective nodes of the \
            //          panel on the left",
            //         TextStyle {
            //             font_size: 25.0,
            //             color: Color::WHITE,
            //             ..default()
            //         },
            //     )
            //     .with_alignment(TextAlignment::Center),
            //     ..Default::default()
            // });
        });
}

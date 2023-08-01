use bevy::prelude::*;

use crate::hud::components::*;
use crate::hud::styles::*;

// pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
//     build_hud(&mut commands, &asset_server);
// }

// pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
pub fn build_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let hud_entity = commands
    commands
        .spawn((
            NodeBundle {
                style: HUD_STYLE,
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            // LHS
            parent
                .spawn(NodeBundle {
                    style: LHS_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Score Text
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new(
                                    "2342340",
                                    get_text_style(&asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ScoreText {},
                    ));
                });
            // RHS
            parent
                .spawn(NodeBundle {
                    style: RHS_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Enemy Text
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    get_text_style(&asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        EnemyText {},
                    ));
                    // Enemy Image
                    // parent.spawn(ImageBundle {
                    //     style: IMAGE_STYLE,
                    //     image: asset_server.load("sprites/ball_red_large.png").into(),
                    //     ..default()
                    // });
                });
        });

    // hud_entity
}
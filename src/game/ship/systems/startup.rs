use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, prelude::{RaycastPickTarget, Pointer, On, Click}};

use crate::game::ship::{client::fetch_my_ships, components::ShipState};

use super::events::ShipSelected;


// #[derive(Component)]
// struct StateIdle {

// }

// #[derive(Component)]
// pub(crate) struct ShipStateMachine<S: Component> {
//     state: S,
// }

// impl ShipStateMachine<StateIdle> {
//     fn new() -> Self {
//         ShipStateMachine {
//             state: StateIdle {}
//         }
//     }
// }


pub(crate) fn add_ships(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ships = fetch_my_ships();
    ships.iter().for_each(|ship| {
        commands.spawn((
            ship.to_owned(),
            SpriteBundle {
                transform: ship.get_transform(),
                texture: asset_server.load("ships/default_frigate.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                ..default()
            },
                        ShipState::new(),
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Click>>::send_event::<ShipSelected>(),
            Name::new(ship.symbol.clone()),
            // PickableBundle::default(),
            // RaycastPickTarget::default(),
            // On::<Pointer<Click>>::send_event::<WaypointSelected>(),
            // Name::new(format!("Waypoint {}", waypoint.symbol))
        ));
        // commands.spawn((
        //     ship.to_owned(),
        //     SpriteBundle {
        //         sprite: Sprite {
        //             color: Color::rgb(0.25, 0.25, 0.75),
        //             custom_size: Some(Vec2::new(ship.get_display_size().0, ship.get_display_size().1)),
        //             ..default()
        //         },
        //         transform: ship.get_transform(),
        //         ..default()
        //     },
            
        //     // bevy_core::Name(ship.symbol.clone()),
        //     // ShipStateMachine::new(),
        //     ShipState::new(),
        //     PickableBundle::default(),
        //     RaycastPickTarget::default(),
        //     On::<Pointer<Click>>::send_event::<ShipSelected>(),
        //     Name::new(ship.symbol.clone()),
        // ));
    })
}

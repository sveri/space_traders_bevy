// use bevy::prelude::*;
use bevy::asset::Handle;
use bevy::prelude::{
    shape, App, Assets, Color, Commands, Component, Entity, Event, EventReader, EventWriter, Mesh,
    PbrBundle, Plugin, Query, Res, ResMut, Resource, StandardMaterial, Transform, Update, Vec3,
    With,
};
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::{On, Pointer};
use bevy_mod_picking::{
    prelude::{Click, RaycastPickTarget},
    PickableBundle,
};

use crate::game::ship::components::Ship;



#[derive(Event, Component, Debug)]
pub(crate) struct ShipSelected(Entity);
// pub(crate) struct ShipSelected(Ship);

impl From<ListenerInput<Pointer<Click>>> for ShipSelected {
    fn from(click_event: ListenerInput<Pointer<Click>>) -> Self {
        // dbg!(click_event.target.);
        Self(click_event.target)
        // ShipSelected()
    }
}

// impl From<ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Click>>> for ShipSelected {
//     fn from(value: ListenerInput<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Click>>) -> Self {
//         todo!()
//     }
// }

// impl bevy_eventlistener::prelude::EntityEvent for ShipSelected {

// }

// impl From<EntityEvent<Click>> for ShipSelected {
//     #[instrument(name = "grid_click", level = "trace", skip_all)]
//     fn from(event: ListenedEvent<Click>) -> Self {
//         GridCellClicked { cell: event.target }
//     }
// }

// impl From<ListenerInput<Pointer<Click>>> for ShipSelected {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         ShipSelected(event.target, event.hit.depth)
//     }
// }

// Unlike callback systems, this is a normal system that can be run in parallel with other systems.
// fn receive_greetings(mut greetings: EventReader<ShipSelected>) {
//     for event in greetings.iter() {
//         info!(
//             "Hello {:?}, you are {:?} depth units away from the pointer",
//             event.0, event.1
//         );
//     }
// }

// #[derive(Event)]
// pub(crate) struct ShipSelected(Entity);


// fn ship_selected(
//     mut ev_levelup: EventReader<ShipSelected>,
// ) {
//     for ev in ev_levelup.iter() {
//         eprintln!("leveled up!");
//         // eprintln!("Entity {:?} leveled up!", ev.0);
//     }
// }
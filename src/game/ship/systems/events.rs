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




// finally was able to use events with bevy 0.11 and patched bevy_mod_picking|_events
// see here for an example: https://github.com/chriamue/flyconomy/blob/53d5b1d91aa39b6b97ff348f22ec35e0f1152a1f/src/game/aerodrome.rs#L33
#[derive(Event, Component, Debug)]
pub(crate) struct ShipSelected(Entity);

impl From<ListenerInput<Pointer<Click>>> for ShipSelected {
    fn from(click_event: ListenerInput<Pointer<Click>>) -> Self {
        Self(click_event.target)
    }
}

pub(crate) fn handle_ship_selected_event(
    mut event: EventReader<ShipSelected>,
    aerodrome_query: Query<(Entity, &Ship)>,
    // mut ev_selected_aerodrome_change: EventWriter<ShipSelected>,
) {
    for select_event in event.iter() {
        if let Ok((_entity, ship)) = aerodrome_query.get(select_event.0) {
            dbg!(ship);
            // ev_selected_aerodrome_change
            //     .send(SelectedAerodromeChangeEvent(aerodrome_component.0.clone()));
        }
    }
}
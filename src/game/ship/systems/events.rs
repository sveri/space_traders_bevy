// use bevy::prelude::*;
use bevy::asset::Handle;
use bevy::prelude::{
    shape, App, Assets, Color, Commands, Component, Entity, Event, EventReader, EventWriter, Mesh, PbrBundle, Plugin, Query, Res,
    ResMut, Resource, StandardMaterial, Transform, Update, Vec3, With,
};
use bevy::text::Text;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::Pointer;
use bevy_mod_picking::prelude::Click;

use crate::game::ship::components::Ship;
use crate::ui::hud::components::SelectedShipText;

// finally was able to use events with bevy 0.11 and patched bevy_mod_picking|_events
// see here for an example: https://github.com/chriamue/flyconomy/blob/53d5b1d91aa39b6b97ff348f22ec35e0f1152a1f/src/game/aerodrome.rs#L33
#[derive(Event, Component, Debug)]
pub(crate) struct ShipSelected(Entity);

impl From<ListenerInput<Pointer<Click>>> for ShipSelected {
    fn from(click_event: ListenerInput<Pointer<Click>>) -> Self { Self(click_event.target) }
}

pub(crate) fn handle_ship_selected_event(
    mut event: EventReader<ShipSelected>,
    ship_query: Query<(Entity, &Ship)>,
    mut select_ship_text: Query<&mut Text, With<SelectedShipText>>, // mut ev_selected_aerodrome_change: EventWriter<ShipSelected>,
) {
    for select_event in event.iter() {
        if let Ok((_entity, ship)) = ship_query.get(select_event.0) {
            select_ship_text.single_mut().sections[0].value = format!("Selected Ship: {}, Status: {}, Fuel: {}", ship.symbol, ship.nav.status, ship.fuel);
        }
    }
}

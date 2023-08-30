use bevy::prelude::*;
use bevy::text::Text;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::Click;
use bevy_mod_picking::prelude::Pointer;

use crate::game::ship::components::Ship;
use crate::ui::controls::components::SelectedShip;
use crate::ui::hud::components::SelectedShipText;

#[derive(Event)]
pub(crate) struct ShipSelected(pub Entity);

impl From<ListenerInput<Pointer<Click>>> for ShipSelected {
    fn from(click_event: ListenerInput<Pointer<Click>>) -> Self { Self(click_event.target) }
}

pub(crate) fn handle_ship_selected_event(mut event: EventReader<ShipSelected>, mut selected_entity: ResMut<SelectedShip>) {
    for select_event in event.iter() {
        selected_entity.0 = Some(select_event.0);
    }
}

pub(crate) fn update_ship_description(
    mut event: EventReader<ShipSelected>, ship_query: Query<(Entity, &Ship)>,
    mut select_ship_text: Query<&mut Text, With<SelectedShipText>>,
) {
    for select_event in event.iter() {
        if let Ok((_entity, ship)) = ship_query.get(select_event.0) {
            select_ship_text.single_mut().sections[0].value =
                format!("Selected Ship: {}, Status: {}, Fuel: {}", ship.symbol, ship.nav.status, ship.fuel);
        }
    }
}

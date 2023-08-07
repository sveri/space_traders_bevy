use bevy::prelude::*;
use bevy::text::Text;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::Click;
use bevy_mod_picking::prelude::Pointer;

use crate::game::waypoint::components::Waypoint;
use crate::ui::controls::components::SelectedWaypoint;
use crate::ui::controls::components::SelectedWaypointText;

// finally was able to use events with bevy 0.11 and patched bevy_mod_picking|_events
// see here for an example: https://github.com/chriamue/flyconomy/blob/53d5b1d91aa39b6b97ff348f22ec35e0f1152a1f/src/game/aerodrome.rs#L33
#[derive(Event, Component, Debug)]
pub(crate) struct WaypointSelected(Entity);

impl From<ListenerInput<Pointer<Click>>> for WaypointSelected {
    fn from(click_event: ListenerInput<Pointer<Click>>) -> Self { Self(click_event.target) }
}

pub(crate) fn handle_waypoint_selected_event(
    mut commands: Commands, mut event: EventReader<WaypointSelected>, waypoint_query: Query<(Entity, &Waypoint)>,
    mut select_waypoint_text: Query<&mut Text, With<SelectedWaypointText>>,
) {
    for select_event in event.iter() {
        if let Ok((_entity, waypoint)) = waypoint_query.get(select_event.0) {
            select_waypoint_text.single_mut().sections[0].value =
                format!("Selected Waypoint: {}, Traits: {}", waypoint.symbol, waypoint.get_traits());
            commands.spawn(SelectedWaypoint {
                waypoint: waypoint.clone(),
            });
        }
    }
}

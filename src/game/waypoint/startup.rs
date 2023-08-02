use bevy::prelude::*;

use crate::st_client;

use super::components::WaypointComponent;




pub(super) fn add_waypoints(mut commands: Commands) {
    let agent_details = st_client::fetch_agent_details();
    let waypoints = st_client::fetch_waypoints(agent_details.get_headquarter_system_symbol().as_str());
    waypoints.iter().for_each(|w| {
        commands.spawn((w.to_owned(), WaypointComponent));
    })
}
use bevy::prelude::*;

struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, (get_agent_details, selected_ship_text, selected_waypoint_text)).add_systems(
        //     Update,
        //     (
        //         show_waypoints.run_if(on_timer(Duration::from_secs_f64(2.0))),
        //         show_ships.run_if(on_timer(Duration::from_secs_f64(1.0))),
        //     ),
        // );
    }
}
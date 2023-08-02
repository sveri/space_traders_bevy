mod systems;
mod components;
mod styles;

use bevy::prelude::*;

use systems::layout::*;

use crate::ship::Ship;


// fn add_people(mut commands: Commands) {
//     commands.spawn((Person, Name("Elaina Proctor".to_string())));
//     commands.spawn((Person, Name("Renzo Hume".to_string())));
//     commands.spawn((Person, Name("Zayna Nieves".to_string())));
// }


pub(crate) struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app
        //     // OnEnter State Systems
            // .add_systems(Update, greet_people);
            .add_systems(Startup, build_hud);
            // .add_systems(spawn_hud.in_schedule(OnEnter(AppState::MainMenu)));
            // Systems
            // .add_systems(
            //     (interact_with_play_button, interact_with_quit_button)
            //         .in_set(OnUpdate(AppState::MainMenu)),
            // )
            // // OnExit State Systems
            // .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
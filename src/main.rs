use std::error::Error;

use bevy::{prelude::*, sprite::Anchor};

use bevy_mod_reqwest::*;

mod ship;
mod st_client;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn setup(mut commands: Commands) { commands.spawn(Camera2dBundle::default()); }

#[derive(Component)]
struct AgentDetailsText;

fn get_agent_details(mut commands: Commands, window: Query<&Window>) {
    let agent_details = st_client::fetch_agent_details();
    let ships = ship::fetch_my_ships();
    dbg!(agent_details.get_headquarter_system_symbol());
    let waypoints = st_client::fetch_waypoints(agent_details.get_headquarter_system_symbol().as_str());
    // println!("{:?}", agent_details);

    // commands.spawn((
    //     // Create a TextBundle that has a Text with a list of sections.
    //     TextBundle::from_sections([TextSection::new(
    //         format!("{}, Faction: {} HQs: {}", agent_details.symbol, agent_details.starting_faction, agent_details.headquarters),
    //         TextStyle {
    //             font_size: 15.0,
    //             color: Color::WHITE,
    //             ..default()
    //         },
    //     )])
    //     .with_style(Style {
    //         left: Val::Px(5.0),
    //         right: Val::Px(5.0),
    //         ..default()
    //     }),
    //     AgentDetailsText,
    // ));

    // commands.spawn((
    //     // Create a TextBundle that has a Text with a list of sections.
    //     TextBundle::from_sections([TextSection::new(
    //         format!("{}, Faction: {} HQs: {}", agent_details.symbol, agent_details.starting_faction, agent_details.headquarters),
    //         TextStyle {
    //             font_size: 15.0,
    //             color: Color::WHITE,
    //             ..default()
    //         },

    //     )])
    //     .with_style(Style {
    //         left: Val::Px(5.0),
    //         right: Val::Px(20.0),
    //         ..default()
    //     }),
    //     AgentDetailsText,
    // ));

    // let slightly_smaller_text_style = TextStyle {
    //     // font,
    //     font_size: 42.0,
    //     color: Color::WHITE,
    //     ..Default::default()
    // };

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            format!(
                "{}, Faction: {} HQs: {}, HQ system symbol: {}",
                agent_details.symbol,
                agent_details.starting_faction,
                agent_details.headquarters,
                agent_details.get_headquarter_system_symbol()
            ),
            TextStyle {
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        }),
    ));

    for (idx, ship) in ships.iter().enumerate() {
        commands.spawn((TextBundle::from_section(
            format!(
                "{} - {}, Location: {}, Crew: {}/{}, Fuel: {}/{}",
                ship.symbol,
                ship.nav.status,
                ship.nav.waypoint_symbol,
                ship.crew.current,
                ship.crew.capacity,
                ship.fuel.current,
                ship.fuel.capacity
            ),
            TextStyle {
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0 * (1 + idx) as f32 + 20.0),
            left: Val::Px(0.0),
            ..default()
        }),));
    }

    for (idx, waypoint) in waypoints.iter().enumerate() {
        commands.spawn((TextBundle::from_section(
            format!("{} - {}", waypoint.symbol, waypoint.get_traits()),
            TextStyle {
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0 * (1 + idx) as f32 + 80.0),
            left: Val::Px(0.0),
            ..default()
        }),));
    }

    // let box_position = Vec2::new(window.single().resolution.width() / -2.0, 0.0);

    // commands.spawn(Text2dBundle {
    //     text: Text {
    //         sections: vec![TextSection::new(
    //             format!(" AnchorasdfasfsafasfsadfasdfasddddddddddddddddSssadf:: "),
    //             TextStyle {
    //                 // color,
    //                 ..slightly_smaller_text_style.clone()
    //             },

    //         )],
    //         ..Default::default()
    //     },
    //     transform: Transform::from_translation(box_position.extend(0.0)),
    //     // transform: Transform::from_xyz(

    //     //     window.single().resolution.width() / -2.0,
    //     //     0.0,
    //     //     // window.single().resolution.height() / -2.0,
    //     //     0.0
    //     // ),
    //     // text_anchor,
    //     ..default()
    // });

    // for (text_anchor, color) in [
    //     (Anchor::TopLeft, Color::RED),
    //     (Anchor::TopRight, Color::GREEN),
    //     (Anchor::BottomRight, Color::BLUE),
    //     (Anchor::BottomLeft, Color::YELLOW),
    // ] {
    //     commands.spawn(Text2dBundle {
    //         text: Text {
    //             sections: vec![TextSection::new(
    //                 format!(" Anchor::{text_anchor:?} "),
    //                 TextStyle {
    //                     color,
    //                     ..slightly_smaller_text_style.clone()
    //                 },
    //             )],
    //             ..Default::default()
    //         },
    //         transform: Transform::from_translation(250. * Vec3::Y),
    //         text_anchor,
    //         ..default()
    //     });
    // }
}

// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in &query {
//             println!("hello {}!", name.0);
//         }
//     }
// }

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ReqwestPlugin)
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, (setup, add_people, get_agent_details))
            // .add_systems(Update, greet_people);
            ;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    App::new().add_plugins((DefaultPlugins, MainPlugin)).run();

    Ok(())
}

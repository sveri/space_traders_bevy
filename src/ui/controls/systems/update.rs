use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub(crate) fn player_camera_control(
    mut mouse_wheel_events: EventReader<MouseWheel>, time: Res<Time>, mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let dist = 5.0 * time.delta().as_secs_f32();

    for mut projection in query.iter_mut() {
        for ev in mouse_wheel_events.iter() {
            let mut log_scale = projection.scale.ln();
            match ev.unit {
                MouseScrollUnit::Line => {
                    if ev.y > 0.0 {
                        log_scale -= dist;
                    } else {
                        log_scale += dist;
                    }
                }
                MouseScrollUnit::Pixel => {
                    println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                }
            }
            projection.scale = log_scale.exp();
        }
    }
}

#[derive(Event)]
pub(crate) struct MPressedEvent;

pub(crate) fn keyboard_input(keys: Res<Input<KeyCode>>,  mut ev_m_pressed: EventWriter<MPressedEvent>) {
    if keys.just_pressed(KeyCode::M) {
        ev_m_pressed.send(MPressedEvent);
    }
    // if keys.just_pressed(KeyCode::Space) {
    //     // Space was pressed
    // }
    // if keys.just_released(KeyCode::LControl) {
    //     // Left Ctrl was released
    // }
    // if keys.pressed(KeyCode::W) {
    //     // W is being held down
    // }
    // // we can check multiple at once with `.any_*`
    // if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
    //     // Either the left or right shift are being held down
    // }
    // if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
    //     // Either delete or backspace was just pressed
    // }
}

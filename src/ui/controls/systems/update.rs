use bevy::prelude::*;

#[derive(Event)]
pub(crate) struct MPressedEvent;

pub(crate) fn keyboard_input(keys: Res<Input<KeyCode>>,  mut ev_m_pressed: EventWriter<MPressedEvent>) {
    if keys.just_pressed(KeyCode::M) {
        ev_m_pressed.send(MPressedEvent);
    }
}

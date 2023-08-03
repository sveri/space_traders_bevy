use bevy::prelude::*;

use crate::ui::hud::components::ErrorText;

pub(in crate::ui::hud) fn clear_error_text(mut error_text: Query<&mut Text, With<ErrorText>>){
    error_text.single_mut().sections[0].value = "".to_string();    
}
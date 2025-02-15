use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub fn cursor_is_locked(primary_window: Query<&Window, With<PrimaryWindow>>) -> bool {
    if let Ok(window) = primary_window.get_single() {
        match window.cursor_options.grab_mode {
            CursorGrabMode::Locked | CursorGrabMode::Confined => true,
            _ => false,
        }
    } else {
        false
    }
}

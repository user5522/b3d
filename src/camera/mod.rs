use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (follow_player, toggle_cursor_lock));
    }
}

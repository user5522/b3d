use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::states::GameState;

pub const CAMERA_DECAY_RATE: f32 = 0.9;
pub const TRAUMA_DECAY_SPEED: f32 = 0.5;

pub const MAX_YAW: f32 = 0.5;
pub const MAX_PITCH: f32 = 0.5;
pub const MAX_ROLL: f32 = 0.5;
pub const MAX_OFFSET: f32 = 500.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenShake>()
            .init_resource::<CameraTilt>()
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (follow_player, screen_shake, camera_tilt).run_if(in_state(GameState::Running)),
            );
    }
}

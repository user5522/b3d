use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::states::GameState;

pub const PLAYER_MESH_RADIUS: f32 = 0.4;
pub const PLAYER_MESH_HEIGHT: f32 = 1.8;
pub const PLAYER_MESH_COLOR: Color = Color::srgb(0.3, 0.9, 0.3);

pub const PLAYER_MOVE_SPEED: f32 = 17.;
pub const PLAYER_JUMP_FORCE: f32 = 8.5;
pub const PLAYER_SLIDE_FORCE: f32 = 17.;
pub const PLAYER_GROUND_SLAM_FORCE: f32 = -50.0;

pub const AIR_FRICTION: f32 = 0.75;
pub const MAX_FALL_SPEED: f32 = -20.0;
pub const GRAVITY: f32 = 9.81;

pub const WALL_CHECK_DISTANCE: f32 = 0.5;
pub const GROUND_CHECK_DISTANCE: f32 = 0.5;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                ground_check,
                wall_check,
                player_movement,
                player_jump,
                update_player_height,
                player_slide,
                player_ground_slam,
            )
                .run_if(in_state(GameState::Running)),
        );
    }
}

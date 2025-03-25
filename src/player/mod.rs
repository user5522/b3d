use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const PLAYER_MESH_RADIUS: f32 = 0.4;
pub const PLAYER_MESH_LENGTH: f32 = 1.8;
pub const PLAYER_MESH_COLOR: Color = Color::srgb(0.3, 0.9, 0.3);

pub const PLAYER_WALK_SPEED: f32 = 4.;
pub const PLAYER_SPRINT_SPEED: f32 = 8.;
pub const PLAYER_JUMP_FORCE: f32 = 5.;
pub const PLAYER_SLIDE_FORCE: f32 = 15.;
pub const PLAYER_GROUND_SLAM_FORCE: f32 = -50.0;

pub const AIR_FRICTION: f32 = 0.75;
const GRAVITY_MULTIPLIER: f32 = 1.25;
const MAX_FALL_SPEED: f32 = -20.0;
const GRAVITY: f32 = 9.81;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                player_movement,
                ground_check,
                player_jump,
                player_slide,
                update_player_height,
                player_ground_slam,
            ),
        );
    }
}

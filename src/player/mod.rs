use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const WALK_SPEED: f32 = 3.;
pub const SPRINT_SPEED: f32 = 6.;
pub const PLAYER_COLOR: Color = Color::srgb(0.3, 0.9, 0.3);
pub const PLAYER_PROPORTIONS: Vec3 = Vec3::new(0.5, 1.0, 0.5);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

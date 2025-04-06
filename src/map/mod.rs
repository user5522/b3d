use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const GRID_COLOR: Color = Color::srgb(1., 1., 1.);
pub const GRID_EMIT_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const GRID_WIDTH: u32 = 100;
pub const GRID_DEPTH: u32 = 100;
pub const GRID_CELL_SIZE: f32 = 2.0;

pub const WALL_HEIGHT: f32 = 10.0;
pub const WALL_WIDTH: f32 = 40.0;
pub const WALL_THICKNESS: f32 = 1.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_lighting, setup_grid, setup_wall));
    }
}

use bevy::prelude::*;

mod systems;

use systems::*;

pub const GRID_COLOR: Color = Color::srgb(1., 1., 1.);
pub const GRID_EMIT_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const GRID_WIDTH: u32 = 100;
pub const GRID_DEPTH: u32 = 100;
pub const GRID_CELL_SIZE: f32 = 2.0;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid);
    }
}

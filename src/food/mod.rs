use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

use crate::states::GameState;

pub const FOOD_COLOR: Color = Color::srgb(0.9, 0.3, 0.3);

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            |commands: Commands,
             meshes: ResMut<Assets<Mesh>>,
             materials: ResMut<Assets<StandardMaterial>>| {
                spawn_food(
                    commands,
                    // default player position, probably should be a constant somewhere
                    &Transform::from_xyz(0.0, 0.5, 0.0),
                    meshes,
                    materials,
                )
            },
        )
        .add_systems(Update, (food_consumed).run_if(in_state(GameState::Running)));
    }
}

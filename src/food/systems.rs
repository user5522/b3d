use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    map::{GRID_CELL_SIZE, GRID_DEPTH, GRID_WIDTH},
    player::components::Player,
};

use super::{components::Food, *};

pub fn spawn_food(
    mut commands: Commands,
    player_transform: &Transform,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::rng();
    let mut new_pos;
    let player_transform = player_transform;

    loop {
        new_pos = Vec2::new(
            rng.random_range(-1 * (GRID_WIDTH / 2) as i32..(GRID_WIDTH / 2) as i32) as f32,
            rng.random_range(-1 * (GRID_DEPTH / 2) as i32..(GRID_DEPTH / 2) as i32) as f32,
        );

        if new_pos
            != Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y,
            )
        {
            break;
        }
    }

    let food_mesh = meshes.add(Cuboid::default());
    let food_material = materials.add(StandardMaterial {
        base_color: FOOD_COLOR,
        ..default()
    });

    commands.spawn((
        Transform::from_xyz(new_pos.x * GRID_CELL_SIZE, 0.5, new_pos.y * GRID_CELL_SIZE),
        GlobalTransform::default(),
        Mesh3d(food_mesh),
        MeshMaterial3d(food_material),
        Food,
    ));
}

pub fn food_consumed(
    mut commands: Commands,
    mut player: Query<(&Transform, &mut Player), With<Player>>,
    mut food: Query<(Entity, &Transform), (With<Food>, Without<Player>)>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let (player_transform, _player) = player.single_mut();

    for (food_entity, food_transform) in food.iter_mut() {
        let distance = player_transform
            .translation
            .distance(food_transform.translation);

        if distance < GRID_CELL_SIZE / 2.0 {
            commands.entity(food_entity).despawn();
            spawn_food(commands, &player_transform, meshes, materials);
            break;
        }
    }
}

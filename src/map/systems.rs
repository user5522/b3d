use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use bevy_rapier3d::prelude::*;

use crate::{
    map::components::Wall,
    physics::{GROUND_FILTER, GROUND_GROUP, WALL_FILTER, WALL_GROUP},
};

use super::{components::Ground, *};

pub fn setup_lighting(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));
}

pub fn setup_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_mesh = meshes.add(Cuboid::new(WALL_WIDTH, WALL_HEIGHT, WALL_THICKNESS));

    commands.spawn((
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GRID_COLOR,
            ..default()
        })),
        Transform::from_xyz(10.0, WALL_HEIGHT / 2.0, 10.0),
        RigidBody::Fixed,
        Sleeping::disabled(),
        Collider::cuboid(WALL_WIDTH / 2.0, WALL_HEIGHT / 2.0, WALL_THICKNESS / 2.0),
        Wall,
        CollisionGroups::new(WALL_GROUP, WALL_FILTER),
    ));
}

pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid_mesh = spawn_grid(GRID_WIDTH, GRID_DEPTH, GRID_CELL_SIZE);

    commands.spawn((
        Mesh3d(meshes.add(grid_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GRID_COLOR,
            emissive: GRID_EMIT_COLOR.into(),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::halfspace(Vec3::Y).unwrap(),
        Ground,
        CollisionGroups::new(GROUND_GROUP, GROUND_FILTER),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(WALL_WIDTH / 2.0, WALL_HEIGHT, WALL_THICKNESS))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GRID_COLOR,
            ..default()
        })),
        Transform::from_xyz(-20.0, WALL_HEIGHT / 2.0, -20.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
        RigidBody::Fixed,
        Sleeping::disabled(),
        Collider::cuboid(WALL_WIDTH / 4.0, WALL_HEIGHT / 2.0, WALL_THICKNESS / 2.0),
        Ground,
        CollisionGroups::new(GROUND_GROUP, GROUND_FILTER),
    ));
}

fn spawn_grid(width: u32, depth: u32, cell_size: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);

    let half_width = (width as f32 * cell_size) / 2.0;
    let half_depth = (depth as f32 * cell_size) / 2.0;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut idx = 0;

    for i in 0..=depth {
        let z = -half_depth + i as f32 * cell_size;

        vertices.push([-half_width, 0.0, z]);
        vertices.push([half_width, 0.0, z]);

        indices.push(idx);
        indices.push(idx + 1);
        idx += 2;
    }

    for i in 0..=width {
        let x = -half_width + i as f32 * cell_size;

        vertices.push([x, 0.0, -half_depth]);
        vertices.push([x, 0.0, half_depth]);

        indices.push(idx);
        indices.push(idx + 1);
        idx += 2;
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(Indices::U32(indices));

    return mesh;
}

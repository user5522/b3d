use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use super::*;

pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid_mesh = spawn_grid(GRID_WIDTH, GRID_DEPTH, GRID_CELL_SIZE);

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));

    commands.spawn((
        Mesh3d(meshes.add(grid_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GRID_COLOR,
            emissive: GRID_EMIT_COLOR.into(),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
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

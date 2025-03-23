use bevy::prelude::*;

use crate::camera::components::MainCamera;

use super::{components::*, *};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_mesh = meshes.add(Cuboid::from_size(PLAYER_PROPORTIONS));
    let player_material = materials.add(StandardMaterial {
        base_color: PLAYER_COLOR,
        ..default()
    });

    commands.spawn((
        Transform::from_xyz(0.0, PLAYER_PROPORTIONS.y / 2.0, 0.0),
        GlobalTransform::default(),
        Mesh3d(player_mesh),
        MeshMaterial3d(player_material),
        Player,
    ));
}

pub fn player_movement(
    mut player: Query<&mut Transform, With<Player>>,
    camera: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = player.single_mut();
    let camera_transform = camera.single();

    let camera_forward = camera_transform.forward().normalize();
    let camera_forward_xz = Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize_or_zero();
    let camera_right = camera_transform.right().normalize();
    let camera_right_xz = Vec3::new(camera_right.x, 0.0, camera_right.z).normalize_or_zero();

    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
        direction += camera_forward_xz;
    }
    if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
        direction -= camera_forward_xz;
    }

    if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
        direction -= camera_right_xz;
    }
    if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
        direction += camera_right_xz;
    }

    let speed = if input.pressed(KeyCode::ShiftLeft) {
        SPRINT_SPEED
    } else {
        WALK_SPEED
    };

    let direction = direction.normalize_or_zero() * speed;

    player_transform.translation += time.delta_secs() * 2.0 * direction;
}

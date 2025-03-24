use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{components::*, *};
use crate::camera::components::MainCamera;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_mesh = meshes.add(Capsule3d::new(PLAYER_MESH_RADIUS, PLAYER_MESH_LENGTH));
    let player_material = materials.add(StandardMaterial {
        base_color: PLAYER_MESH_COLOR,
        ..default()
    });

    commands.spawn((
        Transform::from_xyz(0.0, PLAYER_MESH_LENGTH / 2.0, 0.0),
        GlobalTransform::default(),
        Mesh3d(player_mesh),
        MeshMaterial3d(player_material),
        RigidBody::Dynamic,
        GroundSensor,
        Collider::capsule(
            Vec3::Y * PLAYER_MESH_LENGTH / 2.0,
            Vec3::NEG_Y * PLAYER_MESH_LENGTH / 2.0,
            PLAYER_MESH_RADIUS,
        ),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::all(),
        LockedAxes::ROTATION_LOCKED,
        Sleeping::disabled(),
        Velocity::zero(),
        Player { grounded: false },
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
        PLAYER_SPRINT_SPEED
    } else {
        PLAYER_WALK_SPEED
    };

    let direction = direction.normalize_or_zero() * speed;

    player_transform.translation += time.delta_secs() * 2.0 * direction;
}

pub fn ground_check(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<&mut Player>,
    sensor_query: Query<Entity, With<GroundSensor>>,
) {
    for event in collision_events.read() {
        let (is_collision_start, e1, e2) = match event {
            CollisionEvent::Started(e1, e2, _) => (true, e1, e2),
            CollisionEvent::Stopped(e1, e2, _) => (false, e1, e2),
        };

        if sensor_query.get(*e1).is_ok() || sensor_query.get(*e2).is_ok() {
            if let Ok(mut player) = player_query.get_single_mut() {
                player.grounded = is_collision_start;
            }
        }
    }
}

pub fn player_jump(
    mut player_query: Query<(&mut Velocity, &Player)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut velocity, player)) = player_query.get_single_mut() {
        if keyboard.pressed(KeyCode::Space) && player.grounded {
            velocity.linvel *= AIR_FRICTION;
            velocity.linvel.y = PLAYER_JUMP_FORCE;
        }

        if velocity.linvel.y < 0.0 && !player.grounded {
            velocity.linvel.y -= GRAVITY * GRAVITY_MULTIPLIER * time.delta_secs();
        }

        if velocity.linvel.y < MAX_FALL_SPEED {
            velocity.linvel.y = MAX_FALL_SPEED;
        }
    }
}

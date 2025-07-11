use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{components::*, *};
use crate::{
    camera::{
        MAX_OFFSET, MAX_PITCH, MAX_ROLL, MAX_YAW,
        components::*,
        resources::{CameraTilt, ScreenShake},
    },
    map::components::Ground,
    physics::{PLAYER_FILTER, PLAYER_GROUP, WALL_GROUP},
};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_mesh = meshes.add(Capsule3d::new(PLAYER_MESH_RADIUS, PLAYER_MESH_HEIGHT));
    let player_material = materials.add(StandardMaterial {
        base_color: PLAYER_MESH_COLOR,
        ..default()
    });

    commands.spawn((
        Transform::from_xyz(0.0, PLAYER_MESH_HEIGHT / 2.0, 0.0),
        GlobalTransform::default(),
        Mesh3d(player_mesh),
        MeshMaterial3d(player_material),
        RigidBody::Dynamic,
        GroundSensor,
        Collider::capsule(
            Vec3::Y * PLAYER_MESH_HEIGHT / 2.0,
            Vec3::NEG_Y * PLAYER_MESH_HEIGHT / 2.0,
            PLAYER_MESH_RADIUS,
        ),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::all(),
        LockedAxes::ROTATION_LOCKED,
        Sleeping::disabled(),
        Velocity::zero(),
        Player {
            grounded: false,
            sliding: false,
            slamming: false,
            slide_direction: Vec3::ZERO,
            current_height: PLAYER_MESH_HEIGHT,
            target_height: PLAYER_MESH_HEIGHT,
            on_wall_left: false,
            on_wall_right: false,
            wall_normal: None,
        },
        CollisionGroups::new(PLAYER_GROUP, PLAYER_FILTER),
    ));
}

pub fn player_movement(
    mut player: Query<(&mut Velocity, &Player)>,
    camera: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut velocity, player) = player.single_mut();

    if player.sliding {
        return;
    }

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

    let target_velocity_xz = direction.normalize_or_zero() * PLAYER_MOVE_SPEED;

    velocity.linvel.x = target_velocity_xz.x;
    velocity.linvel.z = target_velocity_xz.z;
}

pub fn wall_check(
    mut player_query: Query<(Entity, &mut Player, &Transform, &Collider)>,
    rapier_context: Query<(
        &RapierQueryPipeline,
        &RapierContextColliders,
        &RapierRigidBodySet,
    )>,
) {
    let Ok((player_entity, mut player, transform, collider)) = player_query.get_single_mut() else {
        return;
    };
    let Ok((query_pipeline, context_colliders, rb_set)) = rapier_context.get_single() else {
        return;
    };

    if player.grounded {
        player.on_wall_left = false;
        player.on_wall_right = false;
        player.wall_normal = None;
        return;
    }

    player.on_wall_left = false;
    player.on_wall_right = false;
    player.wall_normal = None;

    let shape = collider;
    let shape_pos = transform.translation;
    let shape_rot = transform.rotation;

    let options = ShapeCastOptions {
        max_time_of_impact: WALL_CHECK_DISTANCE,
        stop_at_penetration: false,
        compute_impact_geometry_on_penetration: true,
        target_distance: 0.0,
    };

    let filter = QueryFilter::new()
        .exclude_collider(player_entity)
        .groups(CollisionGroups::new(PLAYER_GROUP, WALL_GROUP));

    let check_wall = |direction: Vec3, on_wall: &mut bool| {
        if let Some((_hit_entity, hit)) = query_pipeline.cast_shape(
            context_colliders,
            rb_set,
            shape_pos.into(),
            shape_rot,
            direction.into(),
            shape,
            options,
            filter,
        ) {
            *on_wall = true;
            return Some(hit.details.unwrap().normal1.into());
        }
        None
    };

    if let Some(normal) = check_wall(transform.right().into(), &mut player.on_wall_right) {
        player.wall_normal = Some(normal);
    }

    if let Some(normal) = check_wall(transform.left().into(), &mut player.on_wall_left) {
        if player.wall_normal.is_none() {
            player.wall_normal = Some(normal);
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

        if !player.grounded {
            let gravity_mult = if velocity.linvel.y < 0.0 { 0.8 } else { 1.4 };

            velocity.linvel.y -= GRAVITY * gravity_mult * time.delta_secs();
        }

        if velocity.linvel.y < MAX_FALL_SPEED {
            velocity.linvel.y = MAX_FALL_SPEED;
        }
    }
}

pub fn player_slide(
    mut player_query: Query<(&mut Player, &mut Transform, &mut Velocity)>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    mut tilt: ResMut<CameraTilt>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut player_transform, mut velocity) = player_query.single_mut();
    let camera_transform = camera_query.single();

    if input.just_pressed(KeyCode::ControlLeft)
        && player.grounded
        && !player.sliding
        && !player.slamming
    {
        player.sliding = true;
        player.slide_direction = camera_transform.forward().as_vec3();
        player.slide_direction.y = 0.0;
        player.slide_direction = player.slide_direction.normalize_or_zero();
        player.target_height = PLAYER_MESH_HEIGHT / 2.0;
    }

    if player.sliding {
        let rotation_amount = if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
            0.1 * time.delta_secs()
        } else if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
            -0.1 * time.delta_secs()
        } else {
            0.0
        };

        if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
            tilt.activate(Vec3::Z, 0.05);
        } else if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
            tilt.activate(Vec3::Z, -0.05);
        } else if tilt.is_active {
            tilt.deactivate();
        }

        let rotation = Quat::from_rotation_y(rotation_amount);

        player.slide_direction = rotation
            .mul_vec3(player.slide_direction)
            .normalize_or_zero();

        velocity.linvel = PLAYER_SLIDE_FORCE * player.slide_direction;

        let target_rotation =
            Quat::from_rotation_arc(player_transform.forward().as_vec3(), player.slide_direction);
        player_transform.rotation = player_transform
            .rotation
            .slerp(target_rotation, time.delta_secs() * 10.0);

        if input.just_released(KeyCode::ControlLeft) || !player.grounded {
            player.sliding = false;
            player.target_height = PLAYER_MESH_HEIGHT;
            velocity.linvel = Vec3::ZERO;
        }
    }
}

pub fn ground_check(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<&mut Player>,
    sensor_query: Query<Entity, With<GroundSensor>>,
    ground_query: Query<Entity, With<Ground>>,
) {
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };

    for event in collision_events.read() {
        let (is_collision_start, e1, e2) = match event {
            CollisionEvent::Started(e1, e2, _) => (true, e1, e2),
            CollisionEvent::Stopped(e1, e2, _) => (false, e1, e2),
        };

        let sensor_entity = if sensor_query.get(*e1).is_ok() {
            Some(*e1)
        } else if sensor_query.get(*e2).is_ok() {
            Some(*e2)
        } else {
            None
        };

        if let Some(sensor_entity) = sensor_entity {
            let other_entity = if sensor_entity == *e1 { *e2 } else { *e1 };

            if ground_query.get(other_entity).is_ok() {
                player.grounded = is_collision_start;
            }
        }
    }
}

pub fn update_player_height(
    mut player_query: Query<(&mut Player, &mut Collider, &mut Transform)>,
    time: Res<Time>,
) {
    if let Ok((mut player, mut collider, mut transform)) = player_query.get_single_mut() {
        if player.current_height != player.target_height {
            let original_feet_pos = transform.translation.y - player.current_height / 2.0;

            player.current_height = player
                .current_height
                .lerp(player.target_height, time.delta_secs() * 50.0);

            *collider = Collider::capsule(
                Vec3::Y * player.current_height / 2.0,
                Vec3::NEG_Y * player.current_height / 2.0,
                PLAYER_MESH_RADIUS,
            );

            transform.translation.y = original_feet_pos + player.current_height / 2.0;
        }
    }
}

pub fn player_ground_slam(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Player)>,
    mut screen_shake: ResMut<ScreenShake>,
) {
    if let Ok((mut velocity, mut player)) = player_query.get_single_mut() {
        let was_slamming = player.slamming;

        if input.just_pressed(KeyCode::ControlLeft) && !player.grounded {
            player.slamming = true;
        } else {
            player.slamming = false;
        }

        if player.slamming && !player.grounded {
            velocity.linvel.y = PLAYER_GROUND_SLAM_FORCE;

            if !was_slamming {
                let screen_shake_clone = screen_shake.clone();
                screen_shake.start_shake(
                    MAX_YAW,
                    MAX_ROLL,
                    MAX_PITCH,
                    MAX_OFFSET,
                    screen_shake_clone.trauma + 10.,
                    1.,
                );
            }
        }
    }
}

pub fn reset_tilt(player_query: Query<&Player>, mut tilt: ResMut<CameraTilt>) {
    let player = player_query.single();

    if !player.sliding {
        tilt.deactivate();
    }
}

use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use rand::Rng;

use crate::player::components::Player;

use super::{CAMERA_DECAY_RATE, TRAUMA_DECAY_SPEED, components::*, resources::*};

pub fn setup_camera(mut commands: Commands, mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 10.0, 10.0),
        GlobalTransform::default(),
        MainCamera,
        FpCamera {
            pitch: 0.0,
            yaw: 0.0,
            base_rotation: Quat::IDENTITY,
            current_tilt_angle: 0.0,
            target_tilt_angle: 0.0,
            tilt_direction: Vec3::ZERO,
        },
    ));
}

pub fn follow_player(
    mut camera: Query<(&mut Transform, &mut FpCamera), With<MainCamera>>,
    mut player: Query<(&mut Transform, &mut Player), Without<MainCamera>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    if let Ok((mut camera_transform, mut camera)) = camera.get_single_mut() {
        if let Ok((mut player_transform, player)) = player.get_single_mut() {
            let sensitivity = 0.005;

            for event in mouse_motion.read() {
                camera.pitch -= event.delta.y * sensitivity;
                camera.yaw -= event.delta.x * sensitivity;
            }

            camera.pitch = camera.pitch.clamp(-1.5, 1.5);

            camera_transform.translation =
                player_transform.translation + Vec3::new(0., player.current_height * 0.5, 0.);

            player_transform.rotation = Quat::from_rotation_y(camera.yaw);

            let tilt_rotation =
                Quat::from_axis_angle(camera.tilt_direction, camera.current_tilt_angle);

            camera.base_rotation = Quat::from_rotation_y(camera.yaw)
                * Quat::from_rotation_x(camera.pitch)
                * tilt_rotation;
        }
    }
}

pub fn screen_shake(
    time: Res<Time>,
    mut screen_shake: ResMut<ScreenShake>,
    mut query: Query<(&mut Transform, &FpCamera), With<MainCamera>>,
) {
    let mut rng = rand::rng();
    let shake = screen_shake.trauma * screen_shake.trauma;

    // TODO: use perlin noise maybe
    let yaw = (screen_shake.max_yaw * shake).to_radians() * rng.random_range(-1.0..1.0);
    let roll = (screen_shake.max_roll * shake).to_radians() * rng.random_range(-1.0..1.0);
    let pitch = (screen_shake.max_pitch * shake).to_radians() * rng.random_range(-1.0..1.0);

    if shake > 0.0 && screen_shake.duration > 0.0 {
        if let Ok((mut transform, camera)) = query.get_single_mut() {
            let rotation = Quat::from_rotation_z(roll)
                * Quat::from_rotation_x(pitch)
                * Quat::from_rotation_y(yaw);
            let base_rotation = camera.base_rotation;
            transform.rotation =
                base_rotation.lerp(base_rotation.mul_quat(rotation), CAMERA_DECAY_RATE);

            screen_shake.duration -= time.delta_secs();
        }
    } else {
        if let Ok((mut transform, camera)) = query.get_single_mut() {
            transform.rotation = transform.rotation.lerp(camera.base_rotation, 1.);
        }
    }

    screen_shake.trauma -= TRAUMA_DECAY_SPEED * time.delta_secs();
    screen_shake.trauma = screen_shake.trauma.clamp(0.0, 1.0);
}

pub fn camera_tilt(mut query: Query<&mut FpCamera>, tilt: Res<CameraTilt>) {
    if let Ok(mut camera) = query.get_single_mut() {
        if tilt.is_active {
            camera.tilt_direction = tilt.direction;
            camera.target_tilt_angle = tilt.target_angle;
        } else {
            camera.target_tilt_angle = 0.0;
        }

        camera.current_tilt_angle = camera
            .current_tilt_angle
            .lerp(camera.target_tilt_angle, 0.05);
    }
}

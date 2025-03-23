use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

use crate::player::components::Player;

use super::components::*;

pub fn setup_camera(mut commands: Commands, mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        MainCamera,
        FpCamera {
            pitch: 0.0,
            yaw: 0.0,
        },
    ));
}

pub fn follow_player(
    mut camera: Query<(&mut Transform, &mut FpCamera), With<MainCamera>>,
    mut player: Query<&mut Transform, (With<Player>, Without<MainCamera>)>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    if let Ok((mut camera_transform, mut camera)) = camera.get_single_mut() {
        if let Ok(mut player_transform) = player.get_single_mut() {
            let sensitivity = 0.005;

            for event in mouse_motion.read() {
                camera.pitch -= event.delta.y * sensitivity;
                camera.yaw -= event.delta.x * sensitivity;
            }

            camera.pitch = camera.pitch.clamp(-1.5, 1.5);

            camera_transform.translation = player_transform.translation;

            player_transform.rotation = Quat::from_rotation_y(camera.yaw);

            camera_transform.rotation =
                Quat::from_rotation_y(camera.yaw) * Quat::from_rotation_x(camera.pitch);
        }
    }
}

pub fn toggle_cursor_lock(keys: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if keys.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = windows.get_single_mut() {
            match window.cursor_options.grab_mode {
                CursorGrabMode::Locked => {
                    window.cursor_options.grab_mode = CursorGrabMode::None;
                    window.cursor_options.visible = true;
                }
                _ => {
                    window.cursor_options.grab_mode = CursorGrabMode::Locked;
                    window.cursor_options.visible = false;
                }
            }
        }
    }
}

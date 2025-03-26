use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct FpCamera {
    pub pitch: f32,
    pub yaw: f32,
    pub base_rotation: Quat,
    pub target_tilt_angle: f32,
    pub current_tilt_angle: f32,
    pub tilt_direction: Vec3,
}

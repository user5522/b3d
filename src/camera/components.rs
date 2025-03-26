use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct FpCamera {
    pub pitch: f32,
    pub yaw: f32,
    pub base_rotation: Quat,
}

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub grounded: bool,
    pub sliding: bool,
    pub slide_direction: Vec3,
    pub current_height: f32,
    pub target_height: f32,
}

#[derive(Component)]
pub struct GroundSensor;

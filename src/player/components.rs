use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub grounded: bool,
    pub sliding: bool,
    pub slide_direction: Vec3,
}

#[derive(Component)]
pub struct GroundSensor;

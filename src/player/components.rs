use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub grounded: bool,
}

#[derive(Component)]
pub struct GroundSensor;

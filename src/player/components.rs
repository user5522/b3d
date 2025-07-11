use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub grounded: bool,
    pub sliding: bool,
    pub slamming: bool,
    pub slide_direction: Vec3,
    pub current_height: f32,
    pub target_height: f32,
    pub on_wall_left: bool,
    pub on_wall_right: bool,
    pub wall_normal: Option<Vec3>,
}

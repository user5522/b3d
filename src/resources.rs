use bevy::prelude::*;

#[derive(Resource)]
pub struct Sensitivity {
    pub value: f32,
}

impl Default for Sensitivity {
    fn default() -> Self {
        Self { value: 0.0025 }
    }
}

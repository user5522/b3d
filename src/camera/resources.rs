use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct ScreenShake {
    pub max_yaw: f32,
    pub max_roll: f32,
    pub max_pitch: f32,
    pub max_offset: f32,
    pub trauma: f32,
    pub duration: f32,
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            max_yaw: 0.0,
            max_roll: 0.0,
            max_pitch: 0.0,
            max_offset: 0.0,
            trauma: 0.0,
            duration: 0.0,
        }
    }
}

impl ScreenShake {
    pub fn start_shake(
        &mut self,
        max_yaw: f32,
        max_roll: f32,
        max_pitch: f32,
        max_offset: f32,
        trauma: f32,
        duration: f32,
    ) {
        self.max_yaw = max_yaw;
        self.max_roll = max_roll;
        self.max_pitch = max_pitch;
        self.max_offset = max_offset;
        self.trauma = trauma.clamp(0.0, 1.0);
        self.duration = duration;
    }
}

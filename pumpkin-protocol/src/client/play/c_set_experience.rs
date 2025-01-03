use pumpkin_macros::client_packet;
use serde::Serialize;

#[derive(Serialize)]
#[client_packet("play:set_experience")]
pub struct CSetExperience {
    pub experience_bar: f32,    // Must be 0.0-1.0
    pub level: i32,             // Current level
    pub total_experience: i32,  // Total XP points
}

impl CSetExperience {
    pub fn new(experience_bar: f32, level: i32, total_experience: i32) -> Self {
        Self {
            experience_bar: experience_bar.clamp(0.0, 1.0),
            level: level.max(0),
            total_experience: total_experience.max(0),
        }
    }
}
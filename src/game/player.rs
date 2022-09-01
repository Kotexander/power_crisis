use macroquad::math::{vec2, Rect, Vec2};

use super::HitBox;

pub struct Player {
    hit_box: Rect,
    vel: Vec2,
}
impl Player {
    pub fn new(hit_box: Rect) -> Self {
        let vel = vec2(0.0, 0.0);
        Self { hit_box, vel }
    }

    /// Get a mutable reference to the player's hit box.
    pub fn hit_box_mut(&mut self) -> &mut Rect {
        &mut self.hit_box
    }

    /// Adds velocity to the position with drag.
    pub fn update_pos(&mut self, drag: f32, delta: f32) {
        self.vel *= drag;
        self.hit_box = self.hit_box.offset(self.vel * delta);
    }

    /// adds current velocity with another velocity
    pub fn add_velocity(&mut self, velocity: Vec2) {
        self.vel += velocity;
    }
}

impl HitBox for Player {
    /// Get a reference to the player's hit box.
    fn hit_box(&self) -> &Rect {
        &self.hit_box
    }
}

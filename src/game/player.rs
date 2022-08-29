use macroquad::math::{vec2, Rect, Vec2};

pub struct Player {
    hit_box: Rect,
    vel: Vec2,
}
impl Player {
    pub fn new(hit_box: Rect) -> Self {
        let vel = vec2(0.0, 0.0);
        Self { hit_box, vel }
    }

    /// Get a reference to the player's hit box.
    pub fn hit_box(&self) -> &Rect {
        &self.hit_box
    }

    /// Get a reference to the player's velocity.
    pub fn vel(&self) -> &Vec2 {
        &self.vel
    }

    /// Set the player's velocity.
    pub fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel;
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

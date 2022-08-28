use macroquad::math::{Vec2, vec2};

pub struct Player {
    pos: Vec2,
    size: Vec2,
    vel: Vec2,
}
impl Player {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        let vel = vec2(0.0, 0.0);
        Self { 
            pos, 
            size, 
            vel,

        }
    }

    /// Set the player's pos.
    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    /// Get a reference to the player's pos.
    pub fn pos(&self) -> &Vec2 {
        &self.pos
    }

    /// Get a mutable reference to the player's pos.
    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }

    /// Set the player's size.
    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }

    /// Get a reference to the player's size.
    pub fn size(&self) -> &Vec2 {
        &self.size
    }

    /// Get a mutable reference to the player's size.
    pub fn size_mut(&mut self) -> &mut Vec2 {
        &mut self.size
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
        self.pos += self.vel*delta;
    }

    /// adds current velocity with another velocity
    pub fn add_velocity(&mut self, velocity: Vec2) {
        self.vel += velocity;
    }

}

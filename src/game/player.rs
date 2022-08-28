use macroquad::math::Vec2;

pub struct Player {
    pos: Vec2,
    size: Vec2,
}
impl Player {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
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
}

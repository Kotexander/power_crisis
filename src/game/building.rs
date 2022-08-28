use macroquad::math::Vec2;

pub struct Building {
    pos: Vec2,
    size: Vec2
}

impl Building {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }

    /// Get a reference to the building's pos.
    pub fn pos(&self) -> &Vec2 {
        &self.pos
    }

    /// Get a reference to the building's size.
    pub fn size(&self) -> &Vec2 {
        &self.size
    }
}
use macroquad::math::{Vec2, vec2};

pub struct ElectricalBox {
    pos: Vec2,
    size: Vec2,
    broken: bool,
}
impl ElectricalBox {
    pub fn new(pos: Vec2,) -> Self {
        let vel = vec2(0.0, 0.0);
        Self { 
            pos, 
            size: vec2(10.0, 16.0),
            broken: false
        }
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    /// Get a reference to the elecrical box's pos.
    pub fn pos(&self) -> &Vec2 {
        &self.pos
    }

    /// Get a reference to the elecrical box's broken.
    pub fn broken(&self) -> &bool {
        &self.broken
    }

    /// Get a mutable reference to the elecrical box's pos.
    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }

    /// Set the elecrical box's size.
    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }

    /// Get a reference to the elecrical box's size.
    pub fn size(&self) -> &Vec2 {
        &self.size
    }

    /// Get a mutable reference to the elecrical box's size.
    pub fn size_mut(&mut self) -> &mut Vec2 {
        &mut self.size
    }
 
}

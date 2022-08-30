use macroquad::math::Rect;

use super::HitBox;

pub struct Building {
    hit_box: Rect,
}

impl Building {
    pub fn new(hit_box: Rect) -> Self {
        Self { hit_box }
    }
}

impl HitBox for Building {
    /// Get a reference to the building's hit box.
    fn hit_box(&self) -> &Rect {
        &self.hit_box
    }
}

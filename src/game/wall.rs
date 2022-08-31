use macroquad::math::Rect;

use super::HitBox;

pub struct Wall {
    hit_box: Rect,
}

impl Wall {
    pub fn new(hit_box: Rect) -> Self {
        Self { hit_box }
    }
}

impl HitBox for Wall {
    /// Get a reference to the building's hit box.
    fn hit_box(&self) -> &Rect {
        &self.hit_box
    }
}

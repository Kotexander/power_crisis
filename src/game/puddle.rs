use macroquad::math::Rect;

use super::HitBox;

pub struct Puddle {
    hit_box: Rect,
}
impl Puddle {
    pub fn new(hit_box: Rect) -> Self {
        Self { hit_box }
    }
}

impl HitBox for Puddle {
    /// Get a reference to the puddle's hit box.
    fn hit_box(&self) -> &Rect {
        &self.hit_box
    }
}

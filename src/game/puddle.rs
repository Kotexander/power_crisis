use macroquad::math::Rect;

use super::HitBox;

pub struct Puddle {
    hit_box: Rect,
    time_left: f32,
    pub rotation: f32,
}
impl Puddle {
    pub fn new(hit_box: Rect, time_left: f32, rotation: f32) -> Self {
        Self {
            hit_box,
            time_left,
            rotation,
        }
    }
    pub fn update(&mut self, delta: f32) {
        self.time_left -= delta;
    }

    /// Get a reference to the puddle's time left.
    pub fn time_left(&self) -> f32 {
        self.time_left
    }
}

impl HitBox for Puddle {
    /// Get a reference to the puddle's hit box.
    fn hit_box(&self) -> &Rect {
        &self.hit_box
    }
}

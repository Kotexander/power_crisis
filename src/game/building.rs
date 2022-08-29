use macroquad::math::Rect;

pub struct Building {
    hit_box: Rect,
}

impl Building {
    pub fn new(hit_box: Rect) -> Self {
        Self { hit_box }
    }

    /// Get a reference to the building's hit box.
    pub fn hit_box(&self) -> &Rect {
        &self.hit_box
    }
}

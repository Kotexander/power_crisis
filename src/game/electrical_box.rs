use macroquad::math::Rect;

pub struct ElectricalBox {
    hit_box: Rect,
    broken: bool,
}
impl ElectricalBox {
    pub fn new(hit_box: Rect) -> Self {
        let broken = false;
        Self { hit_box, broken }
    }

    /// Get a reference to the elecrical box's broken.
    pub fn broken(&self) -> &bool {
        &self.broken
    }

    /// Get a reference to the electrical box's hit box.
    pub fn hit_box(&self) -> Rect {
        self.hit_box
    }
}

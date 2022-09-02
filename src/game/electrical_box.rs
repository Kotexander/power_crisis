use macroquad::math::Rect;

use super::HitBox;

#[derive(Copy, Clone)]
pub struct ElectricalBox {
    hit_box: Rect,
    fix_hit_box: Rect,
    broken: bool,
}
impl ElectricalBox {
    pub fn new(hit_box: Rect) -> Self {
        let fix_hit_box = Rect::new(
            hit_box.x - 1.0,
            hit_box.y - 1.0,
            hit_box.w + 2.0,
            hit_box.h + 2.0,
        );
        let broken = false;
        Self {
            hit_box,
            fix_hit_box,
            broken,
        }
    }

    /// Get a reference to the elecrical box's broken.
    pub fn broken(&self) -> &bool {
        &self.broken
    }

    /// Get a mutable reference to the elecrical box's broken.
    pub fn broken_mut(&mut self) -> &mut bool {
        &mut self.broken
    }

    /// Get a reference to the electrical box's fix hit box.
    pub fn fix_hit_box(&self) -> &Rect {
        &self.fix_hit_box
    }
}

impl HitBox for ElectricalBox {
    /// Get a reference to the electrical box's hit box.
    fn hit_box(&self) -> &Rect {
        &self.hit_box
    }
}

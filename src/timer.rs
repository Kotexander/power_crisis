use macroquad::rand::{gen_range};

pub struct Timer {
    min_time: f32,
    max_time: f32,
    rand_time: f32,
    current_time: f32,
}
impl Timer {
    pub fn new(min_time: f32, max_time: f32) -> Self {
        let current_time = 0.0;
        let rand_time = gen_range(min_time, max_time);
        Self {
            min_time,
            max_time,
            rand_time,
            current_time
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.current_time += delta;
    }

    pub fn reset(&mut self) {
        if self.current_time >= self.rand_time {
            self.current_time = 0.0;
            self.rand_time = gen_range(self.min_time, self.max_time);
        }
    }

    /// Returns bool if current time excides the random time
    pub fn is_active(&self) -> bool{
        if self.current_time >= self.rand_time {
            return true;
        }
        return false;
    }
}
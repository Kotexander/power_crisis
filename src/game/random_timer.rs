use macroquad::rand::gen_range;

pub struct RandomTimer {
    min_time: f32,
    max_time: f32,
    time_left: f32,
}
impl RandomTimer {
    pub fn new(min_time: f32, max_time: f32) -> Self {
        let time_left = gen_range(min_time, max_time);
        Self {
            min_time,
            max_time,
            time_left,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.time_left -= delta;
    }

    pub fn reset(&mut self) {
        self.time_left = gen_range(self.min_time, self.max_time);
    }

    /// Returns bool if current time excides the random time
    pub fn is_active(&self) -> bool {
        if self.time_left <= 0.0 {
            return true;
        }
        false
    }
}

use macroquad::math::{vec2, Vec2};
use macroquad::rand::gen_range;

pub struct Lightning {
    origin: Vec2,
    points: Vec<Vec2>,
    max_duration: f32,
    current_duration: f32,

    time_until_change: f32,
    time_between_change: f32,
}
impl Lightning {
    pub fn new(origin: Vec2, max_duration: f32) -> Self {
        let points = Lightning::gen_lightning_points(origin);

        let current_duration = 0.0;

        let time_until_change = 0.0;
        let time_between_change = 0.012;
        Self {
            points,
            max_duration,
            current_duration,
            origin,

            time_until_change,
            time_between_change,
        }
    }

    pub fn gen_lightning_points(origin: Vec2) -> Vec<Vec2> {
        let mut bottom_point = origin;
        let mut top_point = origin;

        let mut points = Vec::new();
        points.push(origin);
        while top_point.y < origin.y + 50.0 {
            let off_y = gen_range(0.2, 1.0);
            let off_x = gen_range(-0.5, 0.5);

            let offset = vec2(off_x, off_y);

            top_point = bottom_point + offset;

            points.push(top_point);
            bottom_point = top_point;
        }

        points
    }

    pub fn points(&self) -> &Vec<Vec2> {
        &self.points
    }

    pub fn new_points(&mut self) {
        if !self.points.is_empty() {
            let points = Lightning::gen_lightning_points(self.origin);
            self.points = points;
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.current_duration += delta;
        self.time_until_change += delta;
        if self.time_until_change >= self.time_between_change {
            self.time_until_change = 0.0;
            self.new_points();
        }
    }

    /// Gives a reference to the ligtning's max duration.
    pub fn max_duration(&self) -> &f32 {
        &self.max_duration
    }

    /// Gives a reference to the ligtning's current duration.
    pub fn current_duration(&self) -> &f32 {
        &self.current_duration
    }

    /// Gives a reference to the ligtning's  time until change.
    pub fn time_until_change(&self) -> f32 {
        self.time_until_change
    }

    /// Gives a reference to the ligtning's time between change.
    pub fn time_between_change(&self) -> f32 {
        self.time_between_change
    }
}

use macroquad::math::{vec2, Vec2};
use macroquad::rand::{gen_range};

pub struct Lightning {
    points: Vec<Vec2>,
    max_duration: f32,
    current_duration: f32,
    origin: Vec2,
}
impl Lightning {
    pub fn new(origin: Vec2, max_duration: f32) -> Self {
        let points = Lightning::gen_lightning_points(origin);

        let current_duration = 0.0;
        Self {
            points,
            max_duration,
            current_duration,
            origin,
        }
    }

    pub fn gen_lightning_points(origin: Vec2) -> Vec<Vec2> {

        let mut bottom_point = origin.clone();
        let mut top_point = origin.clone();

        let mut points = Vec::new();
        points.push(origin);
        while top_point.y < origin.y + 10.0 {
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
        if self.points.len() > 0 {
            let points = Lightning::gen_lightning_points(self.origin);
            self.points = points;
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.current_duration += delta;
        if self.current_duration >= self.max_duration {
            self.current_duration = 0.0;
            self.new_points();
        }
    }
}
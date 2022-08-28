mod generator;
pub use  generator::*;

pub struct Game {
    generator: Generator
}

impl Game {
    pub fn new() -> Self {
        let generator = Generator::new(1.0, 0.1, true);
        Self { generator }
    }
    pub fn update(&mut self, delta: f32) {
        self.generator.update(delta);
    }

    /// Get a reference to the game's generator.
    pub fn generator(&self) -> &Generator {
        &self.generator
    }
}
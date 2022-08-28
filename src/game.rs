mod generator;
pub use generator::*;

mod player;
pub use player::*;

use macroquad::math::vec2;

pub struct Game {
    generator: Generator,
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        let generator = Generator::new(1.0, 0.1, true);
        let player = Player::new(vec2(0.0, 0.0), vec2(1.0, 1.75));
        Self { generator, player }
    }
    pub fn update(&mut self, delta: f32) {
        self.generator.update(delta);
    }

    /// Get a reference to the game's generator.
    pub fn generator(&self) -> &Generator {
        &self.generator
    }

    /// Get a reference to the game's player.
    pub fn player(&self) -> &Player {
        &self.player
    }
}

mod generator;
pub use generator::*;

mod player;
pub use player::*;

mod building;
pub use building::*;

mod electrical_box;
pub use electrical_box::*;

mod lightning;
pub use lightning::*;

use macroquad::math::{vec2, Rect};

use crate::PIXELS_PER_UNIT;

pub struct Game {
    generator: Generator,
    player: Player,
    buildings: Vec<Building>,
    number_of_repair_kits: u32,
    max_number_of_repair_kits: u32,
    electrical_boxes: Vec<ElectricalBox>,
}

impl Game {
    pub fn new() -> Self {
        let generator = Generator::new(1.0, 0.1, true);
        let player = Player::new(Rect::new(0.0, 0.0, 6.0 / PIXELS_PER_UNIT, 12.0/PIXELS_PER_UNIT));

        let buildings = vec![Building::new(Rect::new(-1.0, -1.0, 2.0, 2.0))];

        let electrical_boxes = vec![ElectricalBox::new(Rect::new(-1.0, 3.0, 10.0 / PIXELS_PER_UNIT, 16.0 / PIXELS_PER_UNIT))];

        let max_number_of_repair_kits = 5;
        let number_of_repair_kits = max_number_of_repair_kits;

        Self {
            generator,
            player,
            buildings,
            number_of_repair_kits,
            max_number_of_repair_kits,
            electrical_boxes,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.generator.update(delta);
        self.player.update_pos(0.75, delta)
    }

    /// Get a reference to the game's generator.
    pub fn generator(&self) -> &Generator {
        &self.generator
    }

    /// Get a reference to the game's player.
    pub fn player(&self) -> &Player {
        &self.player
    }

    /// Get a mutable reference to the games's player.
    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    /// Get a reference to the game's buildings.
    pub fn buildings(&self) -> &[Building] {
        self.buildings.as_ref()
    }

    /// Get a reference to the game's number of repair kits.
    pub fn number_of_repair_kits(&self) -> &u32 {
        &self.number_of_repair_kits
    }

    /// Get a mutable reference to the game's number of repair kits.
    pub fn number_of_repair_kits_mut(&mut self) -> &mut u32 {
        &mut self.number_of_repair_kits
    }

    /// Get a reference to the game's max number of repair kits.
    pub fn max_number_of_repair_kits(&self) -> &u32 {
        &self.max_number_of_repair_kits
    }

    /// Get a reference to the game's electricalBoxes.
    pub fn electrical_boxes(&self) -> &[ElectricalBox] {
        self.electrical_boxes.as_ref()
    }

    /// Gives a usize number of how many electrical boxes are still working
    pub fn get_working_boxes(&self) -> usize {
        let mut amount: usize = 0;
        for ebox in self.electrical_boxes.iter() {
            if !ebox.broken() {
                amount += 1;
            }
        }
        amount
    }

}

mod generator;
pub use generator::*;

mod player;
pub use player::*;

mod building;
pub use building::*;

mod electrical_box;
pub use electrical_box::*;

mod puddle;
pub use puddle::*;

use macroquad::math::{Vec2, vec2, Rect};

use crate::PIXELS_PER_UNIT;

pub trait HitBox {
    fn hit_box(&self) -> &Rect;
}

pub struct Game {
    generator: Generator,
    player: Player,

    buildings: Vec<Building>,

    number_of_repair_kits: u32,
    max_number_of_repair_kits: u32,
    electrical_boxes: Vec<ElectricalBox>,

    puddles: Vec<Puddle>
}

impl Game {
    pub fn new() -> Self {
        let generator = Generator::new(1.0, 0.1, true);
        let player = Player::new(Rect::new(0.0, 0.0, 6.0 / PIXELS_PER_UNIT, 6.0/PIXELS_PER_UNIT));

        let buildings = vec![Building::new(Rect::new(-1.0, -1.0, 2.0, 2.0))];

        let electrical_boxes = vec![ElectricalBox::new(Rect::new(-1.0, 3.0, 10.0 / PIXELS_PER_UNIT, 16.0 / PIXELS_PER_UNIT))];

        let max_number_of_repair_kits = 5;
        let number_of_repair_kits = max_number_of_repair_kits;


        let puddles = vec![Puddle::new(Rect::new(3.0, 3.0, 1.0, 1.0))];

        Self {
            generator,
            player,
            buildings,
            number_of_repair_kits,
            max_number_of_repair_kits,
            electrical_boxes,
            puddles,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.generator.update(delta);
        self.player.update_pos(self.which_drag(), delta);
        self.player_collisions();
    }

    fn player_collisions(&mut self ) {
        for i in 0..self.buildings().len() {
            if let Some(v) = aabb_collision(self.player.hit_box(), self.buildings()[i].hit_box()) {
                self.player.hit_box_mut().move_to(v);
            }
        }

        for i in 0..self.electrical_boxes().len() {
            if let Some(v) = aabb_collision(self.player.hit_box(), self.electrical_boxes()[i].hit_box()) {
                self.player.hit_box_mut().move_to(v);
            }
        }
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

    /// Get a reference to the game's puddles.
    pub fn puddles(&self) -> &[Puddle] {
        self.puddles.as_ref()
    }

    fn which_drag(&self) -> f32 {
        for puddle in self.puddles() {
            if puddle.hit_box().overlaps(self.player.hit_box()) {
                // puddle drag
                return 0.5;
            }
        }
        // defualt drag
        0.75
    }
}

fn aabb_collision(first: &Rect, other: &Rect) -> Option<Vec2> {
    if !overlaps(first, other) {
        return None;
    }

    let bottom_in = other.top() - first.bottom();
    let top_in = first.top() - other.bottom();
    
    let left_in = other.left() - first.right();
    let right_in = first.left() - other.right();


    if bottom_in < top_in && bottom_in < left_in && bottom_in < right_in{
        return Some(vec2(first.x, other.bottom()));
    }
    else if top_in < left_in && top_in < right_in {
        return Some(vec2(first.x, other.top() - first.h));
    }
    if left_in < right_in {
        return Some(vec2(other.right(), first.y));
    }
    else {
        return Some(vec2(other.left() - first.w, first.y));
    }
}

pub fn overlaps(first: &Rect, other: &Rect) -> bool {
    first.left() < other.right()
        && first.right() > other.left()
        && first.top() < other.bottom()
        && first.bottom() > other.top()
}
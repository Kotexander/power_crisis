
mod generator;
pub use generator::*;

mod player;
pub use player::*;

mod wall;
pub use wall::*;

mod electrical_box;
pub use electrical_box::*;

mod puddle;
pub use puddle::*;

mod random_timer;
pub use random_timer::*;

use macroquad::{
    math::{vec2, Rect, Vec2},
    rand::{gen_range, RandomRange},
};

use crate::PIXELS_PER_UNIT;

pub trait HitBox {
    fn hit_box(&self) -> &Rect;
}

pub struct Game {
    generator: Generator,
    player: Player,

    walls: Vec<Wall>,

    number_of_repair_kits: u32,
    max_number_of_repair_kits: u32,
    electrical_boxes: Vec<ElectricalBox>,

    puddles: Vec<Puddle>,
    puddle_timer: RandomTimer,

    map_width: f32,
    map_height: f32,
}

impl Game {
    pub fn load() -> Self {
        let map: serde_json::Value =
            // serde_json::from_reader(std::fs::File::open("map.json").unwrap()).unwrap();
            serde_json::from_slice(include_bytes!("../map.json")).unwrap();

        let generator = Generator::new(1.0, 0.1, true);

        let player = &map["player"];
        let player = Player::new(Rect::new(
            player["x"].as_f64().unwrap() as f32,
            player["y"].as_f64().unwrap() as f32,
            6.0 / PIXELS_PER_UNIT,
            6.0 / PIXELS_PER_UNIT,
        ));

        let mut walls = vec![];
        for wall in map["walls"].as_array().unwrap() {
            walls.push(Wall::new(Rect::new(
                wall["x"].as_f64().unwrap() as f32,
                wall["y"].as_f64().unwrap() as f32,
                wall["w"].as_f64().unwrap() as f32,
                wall["h"].as_f64().unwrap() as f32,
            )));
        }

        let electrical_boxes = vec![ElectricalBox::new(Rect::new(
            -1.0,
            3.0,
            10.0 / PIXELS_PER_UNIT,
            16.0 / PIXELS_PER_UNIT,
        ))];

        let puddles = vec![];
        let puddle_timer = RandomTimer::new(0.1, 1.0);

        let max_number_of_repair_kits = 5;
        let number_of_repair_kits = max_number_of_repair_kits;

        let map_width = 1600.0 / PIXELS_PER_UNIT;
        let map_height = 800.0 / PIXELS_PER_UNIT;

        Self {
            generator,
            player,
            walls,
            max_number_of_repair_kits,
            number_of_repair_kits,
            electrical_boxes,
            puddles,
            puddle_timer,
            map_width,
            map_height,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.puddle_timer.update(delta);

        self.generator.update(delta);
        self.update_puddles(delta);
        self.player.update_pos(self.which_drag(), delta);
        self.map_collisions();
        self.player_collisions();

    }

    fn update_puddles(&mut self, delta: f32) {
        if self.puddle_timer.is_active() {
            self.spawn_puddle();
            self.puddle_timer.reset()
        }
        for puddle in &mut self.puddles {
            puddle.update(delta);
        }

        let mut i = 0;
        while i < self.puddles.len() {
            if self.puddles[i].time_left() <= 0.0 {
                self.puddles.remove(i);
            }
            else {
                i += 1;
            }
        }

        // self.puddles.drain_filter(|p| {
            // if p.time_left() >= 0.0 {
                // return true;
            // }
            // false
        // });
        
    }

    fn player_collisions(&mut self) {
        for i in 0..self.walls().len() {
            if let Some(v) = aabb_collision(self.player.hit_box(), self.walls()[i].hit_box()) {
                self.player.hit_box_mut().move_to(v);
            }
        }

        for i in 0..self.electrical_boxes().len() {
            if let Some(v) =
                aabb_collision(self.player.hit_box(), self.electrical_boxes()[i].hit_box())
            {
                self.player.hit_box_mut().move_to(v);
            }
        }
    }

    fn map_collisions(&mut self) {
        if self.player.hit_box().left() < 0.0 {
            let x = 0.0;
            let y = self.player.hit_box().y;
            self.player.hit_box_mut().move_to(vec2(x, y));
        }
        if self.player.hit_box().top() < 0.0 {
            let x = self.player.hit_box().x;
            let y = 0.0;
            self.player.hit_box_mut().move_to(vec2(x, y));
        }
        if self.player.hit_box().right() > self.map_width {
            let x = self.map_width - self.player.hit_box().w;
            let y = self.player.hit_box().y;
            self.player.hit_box_mut().move_to(vec2(x, y));
        }
        if self.player.hit_box().bottom() > self.map_height {
            let x = self.player.hit_box().x;
            let y = self.map_height - self.player.hit_box().h;
            self.player.hit_box_mut().move_to(vec2(x, y));
        }
    }

    fn spawn_puddle(&mut self) {
        let mut hit_box = Rect::new(
            gen_range(0.0, self.map_width - 1.0),
            gen_range(0.0, self.map_height - 1.0),
            1.0,
            1.0,
        );

        for wall in &self.walls {
            if let Some(v) = aabb_collision(&hit_box, &wall.hit_box()) {
                hit_box.move_to(v);
            }
        }
        
        self.puddles.push(Puddle::new(hit_box, 60.0, gen_range(0.0, std::f32::consts::TAU)));
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

    /// Get a reference to the game's walls.
    pub fn walls(&self) -> &[Wall] {
        self.walls.as_ref()
    }

    /// Get a reference to the game's number of repair kits.
    pub fn number_of_repair_kits(&self) -> &u32 {
        &self.number_of_repair_kits
    }

    /// Get a mutable reference to the game's number of repair kits.
    pub fn number_of_repair_kits_mut(&mut self) -> &mut u32 {
        &mut self.number_of_repair_kits
    }

    /// Get a reference to  the game's max number of repair kits.
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

    if bottom_in > top_in && bottom_in > left_in && bottom_in > right_in {
        return Some(vec2(first.x, other.top() - first.h));
    } else if top_in > left_in && top_in > right_in {
        return Some(vec2(first.x, other.bottom()));
    }
    if left_in > right_in {
        return Some(vec2(other.left() - first.w, first.y));
    } else {
        return Some(vec2(other.right(), first.y));
    }
}

pub fn overlaps(first: &Rect, other: &Rect) -> bool {
    first.left() < other.right()
        && first.right() > other.left()
        && first.top() < other.bottom()
        && first.bottom() > other.top()
}

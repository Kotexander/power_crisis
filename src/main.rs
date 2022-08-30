use macroquad::prelude::*;

mod game;
use game::*;

mod lightning;
use lightning::*;

struct Assets {
    player_animation: Vec<Texture2D>,
    electrical_box: Texture2D,
    electrical_box_broken: Texture2D,
    repair_kit: Texture2D,
}
impl Assets {
    async fn load() -> Self {
        let mut player_animation = Vec::new();

        for i in 1..=2 {
            let path = format!("assets/player_idle{}.png", i);
            let frame = load_texture(&path).await.unwrap();
            frame.set_filter(FilterMode::Nearest);
            player_animation.push(frame);
        }

        let electrical_box = load_texture("assets/electrical_box.png").await.unwrap();
        electrical_box.set_filter(FilterMode::Nearest);

        let electrical_box_broken = load_texture("assets/electrical_box_broken.png")
            .await
            .unwrap();
        electrical_box_broken.set_filter(FilterMode::Nearest);

        let repair_kit = load_texture("assets/repair_kit.png").await.unwrap();
        repair_kit.set_filter(FilterMode::Nearest);

        Self {
            player_animation,
            electrical_box,
            electrical_box_broken,
            repair_kit,
        }
    }
}

struct AnimationManager {
    frame_index: usize,
    frames_per_second: f32,
    time_since_last_frame: f32,
    number_of_frames: usize,
}
impl AnimationManager {
    fn new(frames_per_second: f32, animation: &Vec<Texture2D>) -> Self {
        Self {
            frame_index: 0,
            frames_per_second,
            time_since_last_frame: 0.0,
            number_of_frames: animation.len(),
        }
    }

    fn update(&mut self, delta: f32) {
        self.time_since_last_frame += delta;

        if self.time_since_last_frame >= self.frames_per_second {
            self.time_since_last_frame = 0.0;
            self.frame_index += 1;

            if self.frame_index >= self.number_of_frames {
                self.frame_index = 0;
            }
        }
    }
}

struct App {
    game: Game,
    camera: Camera2D,
    assets: Assets,
    player_facing_left: bool,
    player_am: AnimationManager,
    lightning: Lightning,
}
impl App {
    async fn new() -> Self {
        let game = Game::new();

        let scale = 0.1;
        let camera = Camera2D {
            zoom: vec2(1.0 * scale, screen_width() / screen_height() * scale),
            ..Camera2D::default()
        };
        set_camera(&camera);

        let assets = Assets::load().await;

        let player_facing_left = false;

        let player_am = AnimationManager::new(1.0, &assets.player_animation);

        let lightning = Lightning::new(vec2(1.0, 1.0), 0.012);

        Self {
            game,
            camera,
            assets,
            player_facing_left,
            player_am,
            lightning,
        }
    }

    fn draw(&self) {
        clear_background(BLACK);

        self.draw_electical_boxes();
        self.draw_buildings();
        self.draw_player();
        self.draw_lighning();
        self.draw_ui();
        
        // TODO: remove in final release
        if is_key_down(KeyCode::Tab) {
            self.draw_hit_boxes();
        }
    }
    
    fn draw_ui(&self) {
        set_default_camera();

        self.draw_generator_ui();
        self.draw_repair_kit_ui();

        set_camera(&self.camera);
    }

    fn update(&mut self, delta: f32) {
        self.player_key_input();
        self.update_animations(delta);
        self.game.update(delta);
        self.lightning.update(delta);
    }

    fn update_animations(&mut self, delta: f32) {
        self.player_am.update(delta);
    }

    fn player_key_input(&mut self) {
        let mut vel = vec2(0.0, 0.0);
        let mut speed = 2.0;

        if is_key_pressed(KeyCode::Space) {
            if *self.game.number_of_repair_kits() > 0 {
                *self.game.number_of_repair_kits_mut() -= 1;
            }
        }

        if is_key_down(KeyCode::LeftShift) {
            speed *= 1.5
        }

        if is_key_down(KeyCode::D) {
            vel.x += 1.0;
            self.player_facing_left = false;
        }
        if is_key_down(KeyCode::A) {
            vel.x -= 1.0;
            self.player_facing_left = true;
        }
        if is_key_down(KeyCode::W) {
            vel.y += 1.0;
        }
        if is_key_down(KeyCode::S) {
            vel.y -= 1.0;
        }
        self.game
            .player_mut()
            .add_velocity(vel.normalize_or_zero() * speed);
    }

    fn draw_player(&self) {
        let player = self.game.player();

        let texture = &self.assets.player_animation[self.player_am.frame_index];

        let draw_param = DrawTextureParams {
            dest_size: Some(vec2(texture.width() / 16.0, texture.height() / 16.0)),
            flip_x: self.player_facing_left,
            flip_y: true,
            ..DrawTextureParams::default()
        };

        let hit_box = player.hit_box();
        draw_texture_ex(*texture, hit_box.x, hit_box.y, WHITE, draw_param);
    }

    fn draw_building(&self, building: &Building) {
        let hit_box = building.hit_box();
        draw_rectangle(hit_box.x, hit_box.y, hit_box.w, hit_box.h, RED);
    }

    fn draw_buildings(&self) {
        for building in self.game.buildings() {
            self.draw_building(building);
        }
    }

    fn draw_electical_box(&self, electrical_box: &ElectricalBox) {
        let texture: &Texture2D = if *electrical_box.broken() {
            &self.assets.electrical_box_broken
        } else {
            &self.assets.electrical_box
        };

        let draw_param = DrawTextureParams {
            dest_size: Some(vec2(texture.width() / 16.0, texture.height() / 16.0)),
            flip_y: true,
            ..DrawTextureParams::default()
        };

        let hit_box = electrical_box.hit_box();
        draw_texture_ex(*texture, hit_box.x, hit_box.y, WHITE, draw_param);
    }

    fn draw_electical_boxes(&self) {
        for ebox in self.game.electrical_boxes() {
            self.draw_electical_box(ebox);
        }
    }

    fn draw_lighning(&self) {
        let len = self.lightning.points().len();

        for (i, point) in self.lightning.points().iter().enumerate() {
            if i+1 == len {
                break;
            }
            let bottom_point = point;
            let top_point = self.lightning.points()[i+1];

            draw_circle(self.lightning.points()[i].x, self.lightning.points()[i].y, 0.05, BLUE);

            draw_line(bottom_point.x, bottom_point.y, top_point.x, top_point.y, 0.1, BLUE);
        }

    }


    fn draw_generator_ui(&self) {

        draw_rectangle(10., 10., 110., 20., DARKGRAY);
        draw_rectangle(15., 15., 100. * self.game.generator().feul(), 10., YELLOW);

    }

    fn draw_repair_kit_ui(&self) {

        let width = (*self.game.max_number_of_repair_kits() as f32 + 1.0)*25.0+5.0;
        draw_rectangle(10., 40., width, 20., DARKGRAY);

        for i in 1..=*self.game.number_of_repair_kits() {
            let offset = i as f32 * 25.0;
            draw_rectangle(15.0 + offset, 45.0, 20.0, 10., RED);
        }
        draw_texture(self.assets.repair_kit, 15.0, 45.0, WHITE)
    }

    fn draw_hit_box_electrical_box(&self, electrical_box: &ElectricalBox) {
        self.draw_hit_box(electrical_box.hit_box());
    }
    
    fn draw_hit_box_electrical_boxes(&self) {
        for electrical_box in self.game.electrical_boxes() {
            self.draw_hit_box_electrical_box(electrical_box);
        }
    }

    fn draw_hit_box_building(&self, building: &Building) {
        self.draw_hit_box(building.hit_box());
    }
    
    fn draw_hit_box_buildings(&self) {
        for building in self.game.buildings() {
            self.draw_hit_box_building(building);
        }
    }

    fn draw_hit_box_player(&self) {
        self.draw_hit_box(self.game.player().hit_box());
    }

    fn draw_hit_boxes(&self) {
        self.draw_hit_box_buildings();
        self.draw_hit_box_electrical_boxes();
        self.draw_hit_box_player();
    }

    fn draw_hit_box(&self, hit_box: &Rect) {
        draw_rectangle_lines(hit_box.x, hit_box.y, hit_box.w, hit_box.h, 0.1, WHITE);
    }
}

#[macroquad::main("Power Crisis")]
async fn main() {
    let mut app = App::new().await;
    loop {

        // TODO: remove in final release
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        app.draw();
        app.update(get_frame_time());

        next_frame().await
    }
}

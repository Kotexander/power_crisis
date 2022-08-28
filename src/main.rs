use std::default;

use macroquad::prelude::*;

mod game;
use game::*;

struct Assets {
    player_animation: Vec<Texture2D>,
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

        Self {
            player_animation,
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
}
impl App {
    async fn new() -> Self {
        let game = Game::new();

        let scale = 0.1;
        let camera = Camera2D {
            zoom: vec2(1.0 * scale, screen_width() / screen_height() * scale),
            ..Camera2D::default()
        };

        let assets = Assets::load().await;

        let player_facing_left = false;

        let player_am = AnimationManager::new(1.0, &assets.player_animation);

        set_camera(&camera);
        Self { 
            game, 
            camera,
            assets,
            player_facing_left,
            player_am,
        }
    }

    fn draw(&self) {
        self.draw_player();
        self.draw_buildings();
        self.draw_generator_ui();
    }

    fn update(&mut self, delta: f32) {
        self.player_key_input();
        self.update_animations(delta);
        self.game.update(delta);
    }
    
    fn update_animations(&mut self, delta: f32) {
        self.player_am.update(delta);
    }

    fn player_key_input(&mut self) {
        let mut vel = vec2(0.0, 0.0);
        let mut speed = 2.0;

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
        self.game.player_mut().add_velocity(vel.normalize_or_zero() * speed);

    }

    fn draw_player(&self) {
        let player = self.game.player();
        
        let texture = &self.assets.player_animation[self.player_am.frame_index];

        let draw_param = DrawTextureParams {
            dest_size: Some(vec2(texture.width()/16.0, texture.height()/16.0)),
            flip_x: self.player_facing_left,
            flip_y: true,
            ..DrawTextureParams::default()
        };
        draw_texture_ex(*texture, player.pos().x, player.pos().y, WHITE, draw_param);
    }

    fn draw_building(&self, building: &Building) {
        draw_rectangle(
            building.pos().x,
            building.pos().y,
            building.size().x,
            building.size().y,
            RED,
        );
    }

    fn draw_buildings(&self) {
        for building in self.game.buildings() {
            self.draw_building(building);
        }
    }

    fn draw_generator_ui(&self) {
        set_default_camera();

        draw_rectangle(10., 10., 110., 20., DARKGRAY);
        draw_rectangle(15., 15., 100. * self.game.generator().feul(), 10., YELLOW);

        set_camera(&self.camera);
    }

}

#[macroquad::main("Power Crisis")]
async fn main() {
    let mut app = App::new().await;
    loop {
        clear_background(BLACK);

        // TODO: remove in final release
        if is_key_pressed(KeyCode::Escape){
            break;
        }

        app.draw();
        app.update(get_frame_time());


        next_frame().await
    }
}

use std::default;

use macroquad::prelude::*;

mod game;
use game::*;

struct Assets {
    player: Texture2D,
} 
impl Assets {
    async fn load() -> Self {
        let player = load_texture("assets/player_idle1.png").await.unwrap();
        player.set_filter(FilterMode::Nearest);

        Self {
            player,
        }
    }
}

struct App {
    game: Game,
    camera: Camera2D,
    assets: Assets,
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

        set_camera(&camera);
        Self { 
            game, 
            camera,
            assets,
        }
    }

    fn draw(&self) {
        self.draw_player();
        self.draw_buildings();
        self.draw_generator_ui();
    }

    fn update(&mut self, delta: f32) {
        self.player_key_input();
        self.game.update(delta);
    }
    
    fn player_key_input(&mut self) {
        let mut vel = vec2(0.0, 0.0);
        let speed = 2.0;
        if is_key_down(KeyCode::D) {
            vel.x += 1.0;
        }
        if is_key_down(KeyCode::A) {
            vel.x -= 1.0;
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
        
        let draw_param = DrawTextureParams {
            dest_size: Some(vec2(self.assets.player.width()/16.0, self.assets.player.height()/16.0)),
            flip_y: true,
            ..DrawTextureParams::default()
        };
        draw_texture_ex(self.assets.player, player.pos().x, player.pos().y, WHITE, draw_param);
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

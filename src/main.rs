use std::default;

use macroquad::prelude::*;

mod game;
use game::*;

struct App {
    game: Game,
    camera: Camera2D,
}
impl App {
    fn new() -> Self {
        let game = Game::new();

        let scale = 0.1;
        let camera = Camera2D {
            zoom: vec2(1.0 * scale, screen_width() / screen_height() * scale),
            ..Camera2D::default()
        };

        set_camera(&camera);
        Self { game, camera }
    }
    fn draw(&self) {
        self.draw_player();
        self.draw_generator_ui();
    }
    fn update(&mut self, delta: f32) {
        self.game.update(delta);
    }

    fn draw_player(&self) {
        let player = self.game.player();
        draw_rectangle(
            player.pos().x,
            player.pos().y,
            player.size().x,
            player.size().y,
            BLUE,
        );
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
    let mut app = App::new();
    loop {
        clear_background(BLACK);

        app.draw();
        app.update(get_frame_time());

        next_frame().await
    }
}

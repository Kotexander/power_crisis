use macroquad::prelude::*;

mod game;
use game::*;

struct App {
    game: Game
}
impl App {
    fn new() -> Self {
        let game = Game::new();

        Self { game }
    }
}

#[macroquad::main("Power Crisis")]
async fn main() {
    loop {
        clear_background(BLACK);
        
        next_frame().await
    }
}

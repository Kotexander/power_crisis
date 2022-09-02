use macroquad::{
    audio::{load_sound, play_sound, PlaySoundParams, Sound},
    prelude::*,
    rand::gen_range,
};

mod game;
use game::*;

mod lightning;
use lightning::*;

pub const PIXELS_PER_UNIT: f32 = 16.0;

enum GameState {
    Start,
    Running,
    End,
}

struct FootstepManager {
    distance_traveled: f32,
    distance_until_sound: f32,
}
impl FootstepManager {
    fn new(distance_until_sound: f32) -> Self {
        let distance_traveled = 0.0;
        Self {
            distance_traveled,
            distance_until_sound,
        }
    }

    fn update(&mut self, distance: f32) {
        self.distance_traveled += distance.abs();
    }

    fn try_sound(&mut self, sound: &Sound) {
        if self.distance_traveled >= self.distance_until_sound {
            self.distance_traveled = 0.0;

            let sound_param = PlaySoundParams {
                ..PlaySoundParams::default()
            };
            play_sound(*sound, sound_param);
        }
    }
}

struct Assets {
    player_animation: Vec<Texture2D>,
    electrical_box: Texture2D,
    electrical_box_broken: Texture2D,
    repair_kit: Texture2D,
    generator: Texture2D,
    puddle: Texture2D,
    map: Texture2D,
    lightning_sound: Sound,
    walk_sound: Sound,
    repair_sound: Sound,
    restock_sound: Sound,

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

        let generator = load_texture("assets/generator.png").await.unwrap();
        generator.set_filter(FilterMode::Nearest);

        let puddle = load_texture("assets/puddle.png").await.unwrap();
        puddle.set_filter(FilterMode::Nearest);

        let map = load_texture("assets/map.png").await.unwrap();
        map.set_filter(FilterMode::Nearest);

        let lightning_sound = load_sound("assets/lightning.wav").await.unwrap();
        let walk_sound = load_sound("assets/walk.wav").await.unwrap();
        let repair_sound = load_sound("assets/repair.wav").await.unwrap();
        let restock_sound = load_sound("assets/restock.wav").await.unwrap();

        Self {
            player_animation,
            electrical_box,
            electrical_box_broken,
            repair_kit,
            generator,
            puddle,
            map,

            lightning_sound,
            walk_sound,
            repair_sound,
            restock_sound,
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
    player_fm: FootstepManager,
    lightnings: Vec<Lightning>,
    lightning_timer: RandomTimer,

    time_survived: f32,
    score: f32,

    game_state: GameState,
}
impl App {
    async fn new() -> App {
        let game = Game::load();

        let scale = 0.1;
        let camera = Camera2D {
            zoom: vec2(1.0 * scale, screen_width() / screen_height() * scale),
            ..Camera2D::default()
        };
        set_camera(&camera);

        let assets = Assets::load().await;

        let player_facing_left = false;

        let player_am = AnimationManager::new(1.0, &assets.player_animation);

        let lightnings = Vec::new();

        let lightning_timer = RandomTimer::new(0.1, 1.0);

        let player_fm = FootstepManager::new(2.0);

        let time_survived = 0.0;

        let score = 0.0;

        let game_state = GameState::Start;
        Self {
            game,
            camera,
            assets,
            player_facing_left,
            player_am,
            player_fm,
            lightnings,
            lightning_timer,

            time_survived,
            score,

            game_state,
        }
    }

    fn draw(&self) {
        clear_background(BLACK);

        self.draw_map();

        self.draw_electical_boxes();
        self.draw_puddles();
        self.draw_player();
        self.draw_lightnings();

        self.draw_ui();
    }

    fn draw_map(&self) {
        let texture = &self.assets.map;
        let draw_param = DrawTextureParams {
            dest_size: Some(vec2(
                texture.width() / PIXELS_PER_UNIT,
                texture.height() / PIXELS_PER_UNIT,
            )),
            flip_y: true,
            ..DrawTextureParams::default()
        };
        draw_texture_ex(*texture, 0.0, 0.0, WHITE, draw_param);
    }

    fn draw_ui(&self) {
        set_default_camera();

        self.draw_generator_ui();
        self.draw_repair_kit_ui();
        self.draw_score();

        let colour = if !self.lightnings.is_empty() {
            let index = self.lightnings.len() - 1;
            let mut a =
                self.lightnings[index].max_duration() * self.lightnings[index].current_duration();
            if a > 0.5 {
                a = 0.5
            }
            Color::new(0.0, 0.0, 0.2, a)
        } else {
            Color::new(0.0, 0.0, 0.2, 0.5)
        };
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), colour);
        set_camera(&self.camera);
    }

    fn update(&mut self, delta: f32) {
        self.game_events();

        let scale = 0.1;
        let aspect = screen_width()/screen_height();
        self.camera.zoom = vec2(1.0 *scale, aspect*scale);

        self.lightning_timer.update(delta);
        self.player_key_input();
        self.update_animations(delta);
        self.game.update(delta);

        self.update_lighnings(delta);

        let player_center = vec2(
            self.game.player().hit_box().x + self.game.player().hit_box().w / 2.0,
            self.game.player().hit_box().y + self.game.player().hit_box().h / 2.0,
        );

        self.camera.offset = -player_center * self.camera.zoom;
        self.lock_camera();

        if self.lightning_timer.is_active() {
            self.lightning_timer.reset();
        }

        if self.game.generator().feul() <= 0.0 {
            self.game_state = GameState::End;
            return;
        }

        self.update_score(delta);
    }

    fn game_events(&mut self) {
        while let Some(event) = self.game.poll_event() {
            match event {
                GameEvent::Restock => {
                    let sound_params = PlaySoundParams{
                        ..PlaySoundParams::default()
                    };
                    play_sound(self.assets.restock_sound, sound_params);                
                }
                GameEvent::FixEBox(_ebox) => {
                    let sound_params = PlaySoundParams{
                        ..PlaySoundParams::default()
                    };
                    play_sound(self.assets.repair_sound, sound_params);
                    self.score += 200.0;
                }
                GameEvent::DestroyEBox(ebox) => {
                    let x = ebox.hit_box().x + ebox.hit_box().w / 2.0;
                    let y = ebox.hit_box().y + ebox.hit_box().h / 2.0;
                    self.lightnings.push(App::new_lightning(
                        &self.assets.lightning_sound,
                        vec2(x, y),
                        1.0,
                    ));
                }
            }
        }
    }

    fn update_animations(&mut self, delta: f32) {
        self.player_am.update(delta);
    }

    fn update_lighnings(&mut self, delta: f32) {
        let mut i = 0;

        if self.lightning_timer.is_active() {
            let x = gen_range(0.0, 1600.0 / PIXELS_PER_UNIT);
            let y = gen_range(0.0, 800.0 / PIXELS_PER_UNIT);
            self.lightnings.push(App::new_lightning(
                &self.assets.lightning_sound,
                vec2(x, y),
                1.0,
            ));
        }

        while i < self.lightnings.len() {
            self.lightnings[i].update(delta);
            if self.lightnings[i].current_duration() >= self.lightnings[i].max_duration() {
                self.lightnings.remove(i as usize);
                continue;
            }
            i += 1
        }
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

        let amount = vel.normalize_or_zero() * speed;
        self.game.player_mut().add_velocity(amount);

        self.player_fm.update(amount.length() / PIXELS_PER_UNIT);
        self.player_fm.try_sound(&self.assets.walk_sound);
    }

    fn draw_player(&self) {
        let player = self.game.player();

        let texture = &self.assets.player_animation[self.player_am.frame_index];

        let draw_param = DrawTextureParams {
            dest_size: Some(vec2(
                texture.width() / PIXELS_PER_UNIT,
                texture.height() / PIXELS_PER_UNIT,
            )),
            flip_x: self.player_facing_left,
            flip_y: true,
            ..DrawTextureParams::default()
        };

        let hit_box = player.hit_box();
        draw_texture_ex(*texture, hit_box.x, hit_box.y, WHITE, draw_param);
    }

    fn draw_electical_box(&self, electrical_box: &ElectricalBox) {
        let texture: &Texture2D = if *electrical_box.broken() {
            &self.assets.electrical_box_broken
        } else {
            &self.assets.electrical_box
        };

        let draw_param = DrawTextureParams {
            dest_size: Some(vec2(
                texture.width() / PIXELS_PER_UNIT,
                texture.height() / PIXELS_PER_UNIT,
            )),
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

    fn draw_lightning(&self, lightning: &Lightning) {
        let len = lightning.points().len();

        for (i, point) in lightning.points().iter().enumerate() {
            if i + 1 == len {
                break;
            }
            let bottom_point = point;
            let top_point = lightning.points()[i + 1];

            draw_circle(lightning.points()[i].x, lightning.points()[i].y, 0.05, BLUE);

            draw_line(
                bottom_point.x,
                bottom_point.y,
                top_point.x,
                top_point.y,
                0.1,
                BLUE,
            );
        }
    }

    fn draw_lightnings(&self) {
        for lightning in self.lightnings.iter() {
            self.draw_lightning(lightning)
        }
    }

    fn draw_puddle(&self, puddle: &Puddle) {
        let texture = &self.assets.puddle;
        let draw_param = DrawTextureParams {
            dest_size: Some(vec2(
                texture.width() / PIXELS_PER_UNIT,
                texture.height() / PIXELS_PER_UNIT,
            )),
            flip_y: true,
            rotation: puddle.rotation,
            ..DrawTextureParams::default()
        };

        let hit_box = puddle.hit_box();
        draw_texture_ex(*texture, hit_box.x, hit_box.y, WHITE, draw_param);
    }

    fn draw_puddles(&self) {
        for puddle in self.game.puddles() {
            self.draw_puddle(puddle);
        }
    }

    fn draw_generator_ui(&self) {
        draw_rectangle(10., 10., 145., 20., DARKGRAY);
        draw_rectangle(
            30.,
            15.,
            120. * self.game.generator().feul().max(0.0),
            10.,
            YELLOW,
        );
        draw_texture(self.assets.generator, 12.5, 12.5, WHITE);
    }

    fn draw_repair_kit_ui(&self) {
        let width = (*self.game.max_number_of_repair_kits() as f32 + 1.0) * 25.0 + 5.0;
        draw_rectangle(10., 40., width, 20., DARKGRAY);

        for i in 1..=*self.game.number_of_repair_kits() {
            let offset = i as f32 * 25.0;
            draw_rectangle(15.0 + offset, 45.0, 20.0, 10., RED);
        }
        draw_texture(self.assets.repair_kit, 15.0, 45.0, WHITE)
    }

    fn draw_score(&self) {
        draw_text(&format!("Score: {}", self.score + self.get_timer_score()), 10.0, 80.0, 25.0, WHITE);
        draw_text(&format!("Time: {:.2}", self.time_survived), 10.0, 110.0, 25.0, WHITE);
    }

    fn new_lightning(sound: &Sound, origin: Vec2, max_duration: f32) -> Lightning {
        let sound_params = PlaySoundParams {
            ..PlaySoundParams::default()
        };

        play_sound(*sound, sound_params);

        Lightning::new(origin, max_duration)
    }

    fn lock_camera(&mut self) {
        let screen_x = self.camera.screen_to_world(vec2(0.0, 0.0)).x;
        let screen_y = self.camera.screen_to_world(vec2(0.0, screen_height())).y;

        if screen_x < 0.0 {
            self.camera.offset.x = -1.0;
        }
        if screen_y < 0.0 {
            self.camera.offset.y = -1.0;
        }

        let screen_x = self.camera.screen_to_world(vec2(screen_width(), 0.0)).x;
        let screen_y = self.camera.screen_to_world(vec2(0.0, 0.0)).y;

        let width = 1600.0 / PIXELS_PER_UNIT;
        let height = 800.0 / PIXELS_PER_UNIT;

        if screen_x > width {
            self.camera.offset.x = -width * self.camera.zoom.x + 1.0;
        }
        if screen_y > height {
            self.camera.offset.y = -height * self.camera.zoom.y + 1.0;
        }
    }

    fn update_score(&mut self, delta: f32) {
        self.time_survived += delta;
    }

    fn get_timer_score(&self) -> f32 {
        let score = self.time_survived.floor();
        
        score*10.0
    } 
}

#[macroquad::main("Power Crisis")]
async fn main() {
    let mut app = App::new().await;
    
    loop {
        let text_x = screen_width()/4.0;
        let text_y = screen_height()/2.0;
        match &app.game_state {
            GameState::Start => {
                set_default_camera();
                draw_text("A severe thunderstorm is threatening a city!", text_x, text_y - 60.0, 20.0, WHITE);
                draw_text("You are an electrition tasked with keeping the city running!", text_x, text_y - 30.0, 20.0, WHITE);
                draw_text("Use <WASD> to move around", text_x, text_y, 20.0, WHITE);
                draw_text("Get close to the electrical boxes to fix them,", text_x, text_y + 30.0, 20.0, WHITE);
                draw_text("Get close to your van to retock your repair kits,", text_x, text_y + 60.0, 20.0, WHITE);
                draw_text("Don't let the backup generator run out of feul!", text_x, text_y + 90.0, 20.0, WHITE);
                draw_text("Press <ENTER> to Start", text_x, text_y + 120.0, 20.0, WHITE);
                if is_key_pressed(KeyCode::Enter) {
                    app.game_state = GameState::Running;
                    app.score = 0.0;
                    app.time_survived = 0.0;
                    get_frame_time();
                }
            }
            GameState::Running => {
                app.update(get_frame_time());
                app.draw();
            }
            GameState::End => {
                set_default_camera();
                clear_background(BLACK);
                draw_text(&format!("You survived for {:.2} Seconds!", app.time_survived), text_x, text_y-30.0, 20.0, WHITE);
                draw_text(&format!("Your final score is {}!", app.score+app.get_timer_score()), text_x, text_y, 20.0, WHITE);
                draw_text("Press <ENTER> to restart", text_x, text_y + 30.0, 20.0, WHITE);

                if is_key_pressed(KeyCode::Enter) {
                    app.game = Game::load();
                    app.game_state = GameState::Running;
                    app.score = 0.0;
                    app.time_survived = 0.0;
                    get_frame_time();
                }
                
            }
        }

        next_frame().await
    }
}

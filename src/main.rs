use ::rand::{thread_rng, Rng};
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};
use macroquad::prelude::*;

enum GameState {
    Playing,
    GameOver,
}

struct Dino {
    y: f32,
    velocity: f32,
    on_ground: bool,
}

impl Dino {
    fn new() -> Dino {
        Dino {
            y: 300.0,
            velocity: 0.0,
            on_ground: true,
        }
    }

    fn update(&mut self) {
        let dt = get_frame_time();

        if is_key_pressed(KeyCode::Space) && self.on_ground {
            self.velocity = -12.0;
            self.on_ground = false;
        }

        if is_key_released(KeyCode::Space) && self.velocity < -6.0 {
            self.velocity = -6.0;
        }

        self.velocity += 30.0 * dt;
        self.y += self.velocity * dt * 60.0;

        if self.y >= 300.0 {
            self.y = 300.0;
            self.velocity = 0.0;
            self.on_ground = true;
        }
    }

    fn draw(&self, texture: &Texture2D) {
        draw_texture(texture, 50.0, self.y, WHITE);
    }
}

struct Cactus {
    x: f32,
}

impl Cactus {
    fn new() -> Self {
        Self { x: screen_width() }
    }

    fn update(&mut self, speed: f32) {
        self.x -= speed;
    }

    fn draw(&self, texture: &Texture2D) {
        draw_texture(texture, self.x, 300.0, WHITE);
    }

    fn off_screen(&self) -> bool {
        self.x < -20.0
    }
}

#[macroquad::main("Dino Game")]
async fn main() {
    let jump_sound = load_sound("assets/jump.wav").await.unwrap();
    let game_over_sound = load_sound("assets/game_over.wav").await.unwrap();
    let bg_music = load_sound("assets/bg_music.wav").await.unwrap();

    let dino_texture = load_texture("assets/dino.png").await.unwrap();
    let cactus_texture = load_texture("assets/cactus.png").await.unwrap();

    // Musik mit Loop spielen (ohne PlaySoundHandle)
    let mut music_playing = true;
    // Musik mit Loop spielen (ohne Default)
    play_sound(
        &bg_music,
        PlaySoundParams {
            looped: true,
            volume: 0.3,
        },
    );

    let mut dino = Dino::new();
    let mut cacti: Vec<Cactus> = Vec::new();
    let mut spawn_timer = 0.0;
    let mut rng = thread_rng();
    let speed = 5.0;
    let mut score = 0.0_f32;
    let mut game_state = GameState::Playing;

    loop {
        let dt = get_frame_time();
        clear_background(WHITE);

        match game_state {
            GameState::Playing => {
                score += dt;

                if is_key_pressed(KeyCode::Space) && dino.on_ground {
                    play_sound(
                        &jump_sound,
                        PlaySoundParams {
                            looped: false,
                            volume: 1.0,
                        },
                    );
                }

                dino.update();

                for cactus in &mut cacti {
                    cactus.update(speed);
                }

                let collided = cacti
                    .iter()
                    .any(|c| dino_rect(&dino).overlaps(&cactus_rect(c)));

                if collided {
                    play_sound(
                        &game_over_sound,
                        PlaySoundParams {
                            looped: false,
                            volume: 1.0,
                        },
                    );
                    game_state = GameState::GameOver;
                }

                dino.draw(&dino_texture);

                for cactus in &cacti {
                    cactus.draw(&cactus_texture);
                }

                draw_text(
                    &format!("Score: {}", score.floor() as i32),
                    10.0,
                    30.0,
                    30.0,
                    BLACK,
                );

                spawn_timer -= dt;
                if spawn_timer <= 0.0 {
                    cacti.push(Cactus::new());
                    spawn_timer = rng.gen_range(1.0..2.0);
                }

                cacti.retain(|c| !c.off_screen());
            }
            GameState::GameOver => {
                draw_text(
                    "GAME OVER",
                    screen_width() / 2.0 - 80.0,
                    screen_height() / 2.0 - 20.0,
                    50.0,
                    RED,
                );
                draw_text(
                    "Press SPACE to continue",
                    screen_width() / 2.0 - 140.0,
                    screen_height() / 2.0 + 30.0,
                    30.0,
                    BLACK,
                );

                if is_key_pressed(KeyCode::Space) {
                    dino = Dino::new();
                    cacti.clear();
                    spawn_timer = 0.0;
                    score = 0.0;
                    game_state = GameState::Playing;

                    if !music_playing {
                        play_sound(
                            &bg_music,
                            PlaySoundParams {
                                looped: true,
                                volume: 0.3,
                            },
                        );
                        music_playing = true;
                    }
                }
            }
        }

        next_frame().await;
    }
}

fn dino_rect(dino: &Dino) -> Rect {
    Rect::new(50.0, dino.y, 50.0, 50.0)
}

fn cactus_rect(cactus: &Cactus) -> Rect {
    Rect::new(cactus.x, 300.0, 20.0, 40.0)
}

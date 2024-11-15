use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::{io::stdout, thread, time::Duration};

const DINO_SPRITE: &str = "🦖";
const CACTUS_SPRITE: &str = "🌵";
const GROUND: &str = "⣿";
const JUMP_HEIGHT: i32 = 5;
const GRAVITY: f32 = 0.5;
const FRAME_TIME: u64 = 50;

struct Game {
    score: u32,
    dino_y: f32,
    dino_velocity: f32,
    cactus_x: i32,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        Self {
            score: 0,
            dino_y: 0.0,
            dino_velocity: 0.0,
            cactus_x: 40,
            game_over: false,
        }
    }

    fn jump(&mut self) {
        if self.dino_y == 0.0 {
            self.dino_velocity = -2.0;
        }
    }

    fn update(&mut self) {
        // Update dinosaur position
        self.dino_velocity += GRAVITY;
        self.dino_y += self.dino_velocity;

        if self.dino_y > 0.0 {
            self.dino_y = 0.0;
            self.dino_velocity = 0.0;
        }

        // Update cactus position
        self.cactus_x -= 1;
        if self.cactus_x < 0 {
            self.cactus_x = 40;
            self.score += 1;
        }

        // Check collision
        if self.cactus_x == 5 && self.dino_y > -2.0 {
            self.game_over = true;
        }
    }

    fn render(&self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        
        // Draw score
        println!("Score: {}", self.score);
        println!();

        // Draw game area
        for y in -JUMP_HEIGHT..1 {
            for x in 0..50 {
                if y == 0 {
                    print!("{}", GROUND);
                } else if x == 5 && y == self.dino_y as i32 {
                    print!("{}", DINO_SPRITE);
                } else if x == self.cactus_x && y == 0 {
                    print!("{}", CACTUS_SPRITE);
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        if self.game_over {
            println!("\nGame Over! Final score: {}", self.score);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(stdout(), Hide)?;

    let mut game = Game::new();

    while !game.game_over {
        // Handle input
        if poll(Duration::from_millis(FRAME_TIME))? {
            if let Event::Key(key_event) = read()? {
                match key_event.code {
                    KeyCode::Char(' ') | KeyCode::Up => game.jump(),
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        game.update();
        game.render();
        thread::sleep(Duration::from_millis(FRAME_TIME));
    }

    execute!(stdout(), Show)?;
    disable_raw_mode()?;
    Ok(())
}
mod js {
    extern {
        pub fn random() -> f64;
    }
}

use std::boxed::Box;
use std::os::raw::c_void;

mod canvas;
mod digit;
mod entity;
mod keyboard;

use canvas::Canvas;
use digit::draw_digit;
use entity::Entity;
use keyboard::Key;

fn main() {}

const BALL_SPEED: f64 = 0.3;
const PADDLE_SPEED: f64 = 0.4;

const BLACK: u32 = 0xff000000;
const BLUE: u32 = 0xff8080ff;
const RED: u32 = 0xffff8080;
const YELLOW: u32 = 0xffffff80;
const WHITE: u32 = 0xffffffff;

#[derive(PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

#[derive(PartialEq)]
pub enum GameMode {
    Waiting,
    SinglePlayer,
    MultiPlayer,
    Winner(Player),
}

pub struct GameState {
    pub score: (u8, u8),
    pub paddle1: Entity,
    pub paddle2: Entity,
    pub ball: Entity,
    pub canvas: Canvas,
    pub mode: GameMode,
}

impl GameState {
    fn start (&mut self, mode: GameMode) {
        self.score = (0, 0);
        self.mode = mode;
        self.start_ball();
    }

    fn reset(&mut self) {
        self.score = (0, 0);
        self.mode = GameMode::Waiting;
        self.paddle1.y = 208.0;
        self.paddle2.y = 208.0;
        self.reset_ball();
    }

    fn reset_ball(&mut self) {
        self.ball.x = 312.0;
        self.ball.y = 232.0;
        self.ball.dx = 0.0;
        self.ball.dy = 0.0;
    }

    fn start_ball(&mut self) {
        let dir = unsafe { js::random() };

        if dir < 0.25 { self.ball.dx = BALL_SPEED; self.ball.dy = BALL_SPEED; }
        else if dir < 0.5 { self.ball.dx = BALL_SPEED; self.ball.dy = -BALL_SPEED; }
        else if dir < 0.75 { self.ball.dx = -BALL_SPEED; self.ball.dy = BALL_SPEED; }
        else { self.ball.dx = -BALL_SPEED; self.ball.dy = -BALL_SPEED; }
    }
}

#[no_mangle]
pub extern "C" fn init() -> *mut c_void {
    let game_state = Box::<GameState>::new(GameState {
        score: (0, 0),
        paddle1: Entity { x: 16.0, y: 208.0, w: 16, h: 64, dx: 0.0, dy: 0.0 },
        paddle2: Entity { x: 608.0, y: 208.0, w: 16, h: 64, dx: 0.0, dy: 0.0 },
        ball: Entity { x: 312.0, y: 232.0, w: 16, h: 16, dx: 0.0, dy: 0.0 },
        canvas: Canvas::init(640, 480),
        mode: GameMode::Waiting,
    });

    Box::<GameState>::into_raw(game_state) as *mut c_void
}

#[no_mangle]
pub fn step(game_state_ptr: *mut GameState, dt: f64) {
    let game_state = unsafe { game_state_ptr.as_mut().unwrap() };

    game_state.paddle1.dy = 0.0;
    game_state.paddle2.dy = 0.0;

    match game_state.mode {
        GameMode::Winner(_) => {
            if keyboard::is_pressed(Key::Space) { game_state.reset() }
        },
        GameMode::Waiting => {
            if keyboard::is_pressed(Key::Digit1) { game_state.start(GameMode::SinglePlayer) }
            if keyboard::is_pressed(Key::Digit2) { game_state.start(GameMode::MultiPlayer) }
        },
        GameMode::SinglePlayer => {
            if keyboard::is_pressed(Key::KeyA) { game_state.paddle1.dy += PADDLE_SPEED }
            if keyboard::is_pressed(Key::KeyQ) { game_state.paddle1.dy -= PADDLE_SPEED }

            if game_state.ball.dx > 0.0 {
                if game_state.paddle2.y + 8.0 < game_state.ball.y + 8.0 { game_state.paddle2.dy += PADDLE_SPEED }
                if game_state.paddle2.y + 56.0 > game_state.ball.y + 8.0 { game_state.paddle2.dy -= PADDLE_SPEED }
            }
        },
        GameMode::MultiPlayer => {
            if keyboard::is_pressed(Key::KeyA) { game_state.paddle1.dy += PADDLE_SPEED }
            if keyboard::is_pressed(Key::KeyQ) { game_state.paddle1.dy -= PADDLE_SPEED }

            if keyboard::is_pressed(Key::KeyL) { game_state.paddle2.dy += PADDLE_SPEED }
            if keyboard::is_pressed(Key::KeyP) { game_state.paddle2.dy -= PADDLE_SPEED }
        },
    }

    game_state.paddle1.move_entity(dt);
    game_state.paddle1.clamp_vertical(16.0, 480.0 - 16.0);

    game_state.paddle2.move_entity(dt);
    game_state.paddle2.clamp_vertical(16.0, 480.0 - 16.0);

    game_state.ball.move_entity(dt);
    game_state.ball.bounce_vertical(16.0, 480.0 - 16.0);

    if game_state.ball.x <= 0.0 {
        game_state.score.1 += 1;
        game_state.reset_ball();

        if game_state.score.1 >= 9 {
            game_state.mode = GameMode::Winner(Player::Player2);
        } else {
            game_state.start_ball();
        }
    }

    if game_state.ball.x >= 640.0 - 16.0 {
        game_state.score.0 += 1;
        game_state.reset_ball();

        if game_state.score.0 >= 9 {
            game_state.mode = GameMode::Winner(Player::Player1);
        } else {
            game_state.start_ball();
        }
    }

    let paddle = if game_state.ball.dx < 0.0 { &game_state.paddle1 } else { &game_state.paddle2 };
    if game_state.ball.collides_with(paddle) { game_state.ball.dx *= -1.0; game_state.ball.dy += paddle.dy * 0.25; }
}

#[no_mangle]
pub fn draw(game_state_ptr: *mut GameState) {
    let game_state = unsafe { game_state_ptr.as_mut().unwrap() };
    let mut canvas = game_state.canvas;

    let width = canvas.width as f64;
    let height = canvas.height as f64;

    // Draw background
    canvas.fill_rect(0.0, 0.0, width, height, BLACK);

    // Draw court
    canvas.fill_rect(0.0, 0.0, width, 16.0, WHITE);
    canvas.fill_rect(0.0, height - 16.0, width, 16.0, WHITE);

    // Draw midline
    for idx in 0 .. 15 {
        canvas.fill_rect(312.0, 8.0 + (idx * 32) as f64, 16.0, 16.0, WHITE);
    }

    // Draw ball
    canvas.fill_rect(game_state.ball.x, game_state.ball.y, game_state.ball.w as f64, game_state.ball.h as f64, YELLOW);

    // Draw paddles
    canvas.fill_rect(game_state.paddle1.x, game_state.paddle1.y, game_state.paddle1.w as f64, game_state.paddle1.h as f64, BLUE);
    canvas.fill_rect(game_state.paddle2.x, game_state.paddle2.y, game_state.paddle2.w as f64, game_state.paddle2.h as f64, RED);

    // Draw score
    draw_digit(&mut canvas, game_state.score.0, 248.0, 32.0, WHITE);
    draw_digit(&mut canvas, game_state.score.1, 344.0, 32.0, WHITE);

    if game_state.mode == GameMode::Waiting {
        canvas.fill_text("press '1' for", 36.0, 48.0, "16px monospace", BLUE);
        canvas.fill_text("single player", 36.0, 64.0, "16px monospace", BLUE);

        canvas.fill_text("Q -  move up", 36.0, 96.0, "16px monospace", BLUE);
        canvas.fill_text("A - move down", 36.0, 112.0, "16px monospace", BLUE);

        canvas.fill_text("press '2' for", 480.0, 48.0, "16px monospace", RED);
        canvas.fill_text(" multi player", 480.0, 64.0, "16px monospace", RED);

        canvas.fill_text("P -  move up", 480.0, 96.0, "16px monospace", RED);
        canvas.fill_text("L - move down", 480.0, 112.0, "16px monospace", RED);
    }

    if game_state.mode == GameMode::Winner(Player::Player1) {
        canvas.fill_text("WINNER", 178.0, 232.0, "32px monospace", BLUE);
        canvas.fill_text("press 'space' to continue", 56.0, 264.0, "16px monospace", BLUE);
    }

    if game_state.mode == GameMode::Winner(Player::Player2) {
        canvas.fill_text("WINNER", 344.0, 240.0, "32px monospace", RED);
        canvas.fill_text("press 'space' to continue", 344.0, 264.0, "16px monospace", RED);
    }
}

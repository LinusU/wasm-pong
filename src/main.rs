use std::boxed::Box;
use std::os::raw::c_void;

mod canvas;
mod entity;
mod keyboard;

use canvas::Canvas;
use entity::Entity;
use keyboard::Key;

fn main() {}

const BALL_SPEED: f64 = 0.3;
const PADDLE_SPEED: f64 = 0.4;

const BLACK: u32 = 0xff000000;
const BLUE: u32 = 0xff8080ff;
const RED: u32 = 0xffff8080;
const YELLOW: u32 = 0xffffff80;

pub struct GameState {
    pub paddle1: Entity,
    pub paddle2: Entity,
    pub ball: Entity,
    pub canvas: Canvas,
}

#[no_mangle]
pub extern "C" fn init() -> *mut c_void {
    let game_state = Box::<GameState>::new(GameState {
        paddle1: Entity { x: 16.0, y: 32.0, w: 16, h: 64, dx: 0.0, dy: 0.0 },
        paddle2: Entity { x: 608.0, y: 256.0, w: 16, h: 64, dx: 0.0, dy: 0.0 },
        ball: Entity { x: 50.0, y: 50.0, w: 16, h: 16, dx: BALL_SPEED, dy: BALL_SPEED },
        canvas: Canvas::init(640, 480),
    });

    Box::<GameState>::into_raw(game_state) as *mut c_void
}

#[no_mangle]
pub fn step(game_state_ptr: *mut GameState, dt: f64) {
    let game_state = unsafe { game_state_ptr.as_mut().unwrap() };

    game_state.paddle1.dy = 0.0;
    if keyboard::is_pressed(Key::KeyA) { game_state.paddle1.dy += PADDLE_SPEED }
    if keyboard::is_pressed(Key::KeyQ) { game_state.paddle1.dy -= PADDLE_SPEED }

    game_state.paddle2.dy = 0.0;
    if keyboard::is_pressed(Key::KeyL) { game_state.paddle2.dy += PADDLE_SPEED }
    if keyboard::is_pressed(Key::KeyP) { game_state.paddle2.dy -= PADDLE_SPEED }

    game_state.paddle1.move_entity(dt);
    game_state.paddle1.bounce_inside_box();
    game_state.paddle2.move_entity(dt);
    game_state.paddle2.bounce_inside_box();

    game_state.ball.move_entity(dt);
    game_state.ball.bounce_inside_box();

    let paddle = if game_state.ball.dx < 0.0 { &game_state.paddle1 } else { &game_state.paddle2 };
    if game_state.ball.collides_with(paddle) { game_state.ball.dx *= -1.0; }
}

#[no_mangle]
pub fn draw(game_state_ptr: *mut GameState) {
    let game_state = unsafe { game_state_ptr.as_mut().unwrap() };
    let mut canvas = game_state.canvas;

    let width = canvas.width;
    let height = canvas.height;

    // Draw background
    canvas.fill_rect(0.0, 0.0, width as f64, height as f64, BLACK);

    // Draw paddles
    canvas.fill_rect(game_state.paddle1.x, game_state.paddle1.y, game_state.paddle1.w as f64, game_state.paddle1.h as f64, BLUE);
    canvas.fill_rect(game_state.paddle2.x, game_state.paddle2.y, game_state.paddle2.w as f64, game_state.paddle2.h as f64, RED);

    // Draw ball
    canvas.fill_rect(game_state.ball.x, game_state.ball.y, game_state.ball.w as f64, game_state.ball.h as f64, YELLOW);
}

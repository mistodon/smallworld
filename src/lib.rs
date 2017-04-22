extern crate find_folder;
#[macro_use] extern crate glium;
extern crate image;
extern crate specs;

use glium::{DisplayBuild};
use glium::glutin::{Event, WindowBuilder, VirtualKeyCode, ElementState};

use std::time::{Instant};

pub mod assets;
pub mod game;
pub mod game_state;
pub mod macros;
pub mod rendering;
pub mod state;
pub mod systems;
pub mod vectors;

use game::Game;
use game_state::GameState;
use rendering::Display;
use state::State;

pub fn run_game(scale: u32)
{
    let virtual_res = (256, 144);
    let display = WindowBuilder::new()
        .with_title("Small World")
        .with_dimensions(virtual_res.0 * scale, virtual_res.1 * scale)
        .with_depth_buffer(24)
        .with_vsync()
        .build_glium()
        .unwrap();

    let mut game = Game::new(virtual_res);

    loop
    {
        let keep_going = run_state(&display, &mut game);
        if !keep_going
        {
            break;
        }
    }
}

pub fn run_state(display: &Display, game: &mut Game) -> bool
{
    let mut game_state = GameState::new(display);
    let mut previous_frame_time = Instant::now();

    loop
    {
        let current_time = Instant::now();
        let delta = current_time.duration_since(previous_frame_time);
        let dt = (delta.as_secs() as f64) + (delta.subsec_nanos() as f64) / 1000_000_000.0;
        previous_frame_time = current_time;

        let mut quitting = false;

        {
            for event in display.poll_events()
            {
                match event
                {
                    Event::KeyboardInput(state, _, Some(key)) =>
                        match state
                        {
                            ElementState::Pressed =>
                                match key
                                {
                                    VirtualKeyCode::Left => game.input.left = true,
                                    VirtualKeyCode::Right => game.input.right = true,
                                    VirtualKeyCode::Up => game.input.up = true,
                                    VirtualKeyCode::Down => game.input.down = true,
                                    VirtualKeyCode::Escape => quitting = true,
                                    _ => ()
                                },
                            ElementState::Released =>
                                match key
                                {
                                    VirtualKeyCode::Left => game.input.left = false,
                                    VirtualKeyCode::Right => game.input.right = false,
                                    VirtualKeyCode::Up => game.input.up = false,
                                    VirtualKeyCode::Down => game.input.down = false,
                                    _ => ()
                                }
                        },
                    Event::Closed => quitting = true,
                    _ => ()
                }
            }
        }

        if quitting
        {
            return false;
        }

        let state_continue = game_state.update(dt, game);
        if !state_continue
        {
            return true;
        }

        let mut target = display.draw();
        game_state.draw(&mut target, game);
        target.finish().expect("Drawing failed");
    }
}

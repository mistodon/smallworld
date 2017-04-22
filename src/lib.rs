#[macro_use] extern crate glium;

use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, WindowBuilder};

use std::time::{Instant};

pub mod state;
pub mod game_state;

use state::State;
use game_state::GameState;

pub fn run_game()
{
    let resolution = (256 * 3, 144 * 3);
    let display = WindowBuilder::new()
        .with_title("Small World")
        .with_dimensions(resolution.0, resolution.1)
        .with_depth_buffer(24)
        .with_vsync()
        .build_glium()
        .unwrap();

    let mut game_state = GameState::new();
    let mut previous_frame_time = Instant::now();

    loop
    {
        let current_time = Instant::now();
        let delta = current_time.duration_since(previous_frame_time);
        let dt = (delta.as_secs() as f64) + (delta.subsec_nanos() as f64) / 1000_000_000.0;
        previous_frame_time = current_time;

        for event in display.poll_events()
        {
            match event
            {
                Event::Closed => return,
                _ => ()
            }
        }

        let keep_going = game_state.update(dt);
        if !keep_going
        {
            return;
        }

        let mut target = display.draw();
        game_state.draw(&mut target);
        target.finish().expect("Drawing failed");
    }
}

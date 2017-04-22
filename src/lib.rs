#[macro_use] extern crate glium;

use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, WindowBuilder};


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

    loop
    {
        for event in display.poll_events()
        {
            match event
            {
                Event::Closed => return,
                _ => ()
            }
        }

        let mut target = display.draw();
        target.clear_color_srgb_and_depth((0.0, 0.0, 0.1, 1.0), 1.0);
        target.finish().expect("Drawing failed");
    }
}

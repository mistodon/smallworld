use glium::{self, DrawParameters, DepthTest, Depth};

use assets::{get_asset_string};
use rendering::*;
use state::*;

pub struct GameState
{
    shader: Shader,
    mesh: Mesh,
    time: f64
}

impl State for GameState
{
    fn new(display: &Display) -> Self
    {
        let shader = load_shader(display, &get_asset_string("shaders/sprite.vs"), &get_asset_string("shaders/sprite.fs"));
        let mesh = quad_mesh(display);
        GameState
        {
            shader: shader,
            mesh: mesh,
            time: 0.0
        }
    }

    fn update(&mut self, dt: f64) -> bool
    {
        self.time += dt;
        self.time < 1.0
    }

    fn draw(&mut self, target: &mut Frame)
    {
        target.clear_color_srgb_and_depth((0.0, 0.0, self.time as f32, 1.0), 1.0);
        target.draw(
            &self.mesh.0,
            &self.mesh.1,
            &self.shader,
            &uniform!
            {

            },
            &DrawParameters
            {
                depth: Depth
                {
                    test: DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            }).unwrap();
    }
}

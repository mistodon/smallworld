use glium::{DrawParameters, DepthTest, Depth, Blend};
use glium::uniforms::{Sampler, MinifySamplerFilter, MagnifySamplerFilter, SamplerWrapFunction};

use assets::{get_asset_string, get_asset_bytes};
use game::Game;
use rendering::*;
use state::*;

pub struct GameState
{
    shader: Shader,
    mesh: Mesh,
    texture: Texture,
    time: f64
}

impl State for GameState
{
    fn new(display: &Display) -> Self
    {
        let shader = load_shader(display, &get_asset_string("shaders/sprite.vs"), &get_asset_string("shaders/sprite.fs"));
        let mesh = quad_mesh(display);
        let texture = load_texture(display, &get_asset_bytes("atlas.png"));
        GameState
        {
            shader: shader,
            mesh: mesh,
            texture: texture,
            time: 0.0
        }
    }

    fn update(&mut self, dt: f64, game: &mut Game) -> bool
    {
        self.time += dt;
        self.time < 1.0
    }

    fn draw(&mut self, target: &mut Frame, game: &mut Game)
    {
        target.clear_color_srgb_and_depth((0.0, 0.0, self.time as f32, 1.0), 1.0);

        let colormap = Sampler::new(&self.texture)
            .minify_filter(MinifySamplerFilter::Nearest)
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .wrap_function(SamplerWrapFunction::Clamp);

        let projection = calculate_projection(game.resolution, game.tile_size);

        target.draw(
            &self.mesh.0,
            &self.mesh.1,
            &self.shader,
            &uniform!
            {
                projection: projection,
                colormap: colormap
            },
            &DrawParameters
            {
                depth: Depth
                {
                    test: DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                blend: Blend::alpha_blending(),
                .. Default::default()
            }).unwrap();
    }
}

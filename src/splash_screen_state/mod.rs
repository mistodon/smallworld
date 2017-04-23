use glium::{DrawParameters, DepthTest, Depth, Blend};
use glium::uniforms::{Sampler, MinifySamplerFilter, MagnifySamplerFilter, SamplerWrapFunction};

use assets::{get_asset_string, get_asset_bytes};
use game::Game;
use rendering::*;
use state::*;

pub struct SplashScreenState
{
    shader: Shader,
    mesh: Mesh,
    splash_texture: Texture,
    time: f64
}

impl State for SplashScreenState
{
    fn new(display: &Display, game: &mut Game) -> Self
    {
        let shader = load_shader(display, &get_asset_string("shaders/splash.vs"), &get_asset_string("shaders/splash.fs"));
        let mesh = quad_mesh(display);
        let splash_texture_name = if game.current_state == StateType::EndingState { "ending_screen.png" } else { "splash_screen.png" };
        let splash_texture = load_texture(display, &get_asset_bytes(splash_texture_name)).0;

        SplashScreenState
        {
            shader: shader,
            mesh: mesh,
            splash_texture: splash_texture,
            time: 0.0
        }
    }

    fn update(&mut self, dt: f64, game: &mut Game) -> bool
    {
        self.time += dt;

        let exiting_state = game.input.any_key_pressed;
        if exiting_state
        {
            game.current_state = match game.current_state
            {
                StateType::SplashScreen => StateType::GameState,
                _ => StateType::SplashScreen
            }
        }

        !exiting_state
    }

    fn draw(&mut self, target: &mut Frame, _game: &mut Game)
    {
        target.clear_color_srgb_and_depth((0.75, 0.75, 0.75, 1.0), 1.0);

        let colormap = Sampler::new(&self.splash_texture)
            .minify_filter(MinifySamplerFilter::Nearest)
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .wrap_function(SamplerWrapFunction::Clamp);

        target.draw(
            &self.mesh.0,
            &self.mesh.1,
            &self.shader,
            &uniform!
            {
                colormap: colormap
            },
            &DrawParameters
            {
                depth: Depth
                {
                    test: DepthTest::IfLess,
                    write: false,
                    .. Default::default()
                },
                blend: Blend::alpha_blending(),
                .. Default::default()
            }).unwrap();
    }
}

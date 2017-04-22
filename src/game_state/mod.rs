use glium::{DrawParameters, DepthTest, Depth, Blend};
use glium::uniforms::{Sampler, MinifySamplerFilter, MagnifySamplerFilter, SamplerWrapFunction};
use specs::{World, Planner, Join, Gate};

use assets::{get_asset_string, get_asset_bytes};
use game::Game;
use rendering::*;
use state::*;
use systems::*;
use vectors::*;

pub struct GameState
{
    shader: Shader,
    mesh: Mesh,
    texture: Texture,
    planner: Planner<()>,
    time: f64
}

impl State for GameState
{
    fn new(display: &Display) -> Self
    {
        let shader = load_shader(display, &get_asset_string("shaders/sprite.vs"), &get_asset_string("shaders/sprite.fs"));
        let mesh = quad_mesh(display);
        let texture = load_texture(display, &get_asset_bytes("atlas.png"));

        let mut world = World::new();
        world.register::<Position>();
        world.register::<Sprite>();
        world.register::<Motion>();

        world.create_now()
            .with(Position(vec2(0.0, 0.0)))
            .with(Sprite)
            .build();

        world.create_now()
            .with(Position(vec2(2.0, 0.0)))
            .with(Sprite)
            .with(Motion::default())
            .build();

        world.create_now()
            .with(Position(vec2(4.0, 0.0)))
            .with(Sprite)
            .with(Motion { destination: Some(motion::Destination { position: vec2(4.0, 4.0), direction: vec2(0.0, 1.0) }) })
            .build();

        let planner = Planner::new(world);

        GameState
        {
            shader: shader,
            mesh: mesh,
            texture: texture,
            planner: planner,
            time: 0.0
        }
    }

    fn update(&mut self, dt: f64, game: &mut Game) -> bool
    {
        self.time += dt;

        self.planner.run_custom(move |arg| motion::move_towards_destinations(arg, dt));

        self.planner.wait();

        true
    }

    fn draw(&mut self, target: &mut Frame, game: &mut Game)
    {
        target.clear_color_srgb_and_depth((0.0, 0.0, self.time as f32, 1.0), 1.0);

        let colormap = Sampler::new(&self.texture)
            .minify_filter(MinifySamplerFilter::Nearest)
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .wrap_function(SamplerWrapFunction::Clamp);

        let projection = calculate_projection(game.resolution, game.tile_size);

        {
            let world = self.planner.mut_world();
            let (position, sprite) = (world.read::<Position>().pass(), world.read::<Sprite>().pass());
            for (position, sprite) in (&position, &sprite).join()
            {
                target.draw(
                    &self.mesh.0,
                    &self.mesh.1,
                    &self.shader,
                    &uniform!
                    {
                        projection: projection,
                        colormap: colormap,
                        position: position.0.components
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
    }
}

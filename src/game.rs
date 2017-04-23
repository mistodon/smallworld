use assets::{self, Level};
use state::{StateType};
use vectors::*;

pub struct Game
{
    pub resolution: (u32, u32),
    pub tile_size: u32,
    pub input: GameInput,
    pub current_state: StateType,
    pub levels: Vec<Level>,
    pub current_level: usize,
    pub complete: bool
}

#[derive(Default)]
pub struct GameInput
{
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub any_key_pressed: bool
}

impl Game
{
    pub fn new(resolution: (u32, u32)) -> Self
    {
        Game
        {
            resolution: resolution,
            tile_size: 16,
            input: GameInput::default(),
            current_state: StateType::SplashScreen,
            levels: assets::load_levels("levels.yaml"),
            current_level: 0,
            complete: false
        }
    }
}

impl GameInput
{
    pub fn dx(&self) -> f32
    {
        if self.left { -1.0 }
        else if self.right { 1.0 }
        else { 0.0 }
    }

    pub fn dy(&self) -> f32
    {
        if self.down { -1.0 }
        else if self.up { 1.0 }
        else { 0.0 }
    }

    pub fn dir(&self) -> Vector2<f32>
    {
        if self.left || self.right { vec2(self.dx(), 0.0) }
        else { vec2(0.0, self.dy()) }
    }
}

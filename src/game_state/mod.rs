use state::*;

pub struct GameState
{
    time: f64
}

impl State for GameState
{
    fn new() -> Self
    {
        GameState { time: 0.0 }
    }

    fn update(&mut self, dt: f64) -> bool
    {
        self.time += dt;
        self.time < 1.0
    }

    fn draw(&mut self, target: &mut Frame)
    {
        target.clear_color_srgb_and_depth((0.0, 0.0, self.time as f32, 1.0), 1.0);
    }
}

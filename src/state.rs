pub use glium::{Frame, Surface};

pub trait State
{
    fn new() -> Self;
    fn update(&mut self, dt: f64) -> bool;
    fn draw(&mut self, target: &mut Frame);
}

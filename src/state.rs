pub use glium::{Frame, Surface};

use rendering::*;

pub trait State
{
    fn new(display: &Display) -> Self;
    fn update(&mut self, dt: f64) -> bool;
    fn draw(&mut self, target: &mut Frame);
}

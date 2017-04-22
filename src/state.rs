pub use glium::{Frame, Surface};

use game::Game;
use rendering::*;

pub trait State
{
    fn new(display: &Display) -> Self;
    fn update(&mut self, dt: f64, game: &mut Game) -> bool;
    fn draw(&mut self, target: &mut Frame, game: &mut Game);
}

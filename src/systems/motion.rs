use specs::{RunArg, Join};

use systems::{Position};
use vectors::*;

#[derive(Default)]
pub struct Motion
{
    pub destination: Option<Destination>
}
component!(Motion);

#[derive(Copy, Clone)]
pub struct Destination
{
    pub position: Vector2<f32>,
    pub direction: Vector2<f32>
}


pub fn move_towards_destinations(arg: RunArg, dt: f64)
{
    let (mut position, mut motion) = arg.fetch(|w| (w.write::<Position>(), w.write::<Motion>()));
    let dt = dt as f32;

    for (position, motion) in (&mut position, &mut motion).join()
    {
        let destination = motion.destination;
        if let Some(destination) = destination
        {
            let vel = destination.direction;
            let new_pos = position.0 + (vel * dt);
            let dest = destination.position;
            let new_disp = dest - new_pos;
            let diverging = new_disp.dot(vel) < 0.0;
            if diverging
            {
                position.0 = dest;
                motion.destination = None;
            }
            else
            {
                position.0 = new_pos;
            }
        };
    }
}

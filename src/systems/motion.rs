use std::collections::VecDeque;

use specs::{RunArg, Join};

use systems::{Position};
use vectors::*;

#[derive(Default)]
pub struct Motion
{
    pub destination: Option<Destination>,
    pub speed: f32
}
component!(Motion);

#[derive(Copy, Clone)]
pub struct Destination
{
    pub position: Vector2<f32>,
    pub direction: Vector2<f32>
}

#[derive(Default)]
pub struct Player
{
    pub moves: u32
}
component!(Player);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Collision
{
    Passable,
    Pushable,
    Obstacle
}
component!(Collision);

pub struct PlayerTracker
{
    pub steps: VecDeque<Vector2<i32>>,
    pub moves: u32
}
component!(PlayerTracker);

impl PlayerTracker
{
    pub fn new<T>(steps: T) -> Self
        where T: Into<VecDeque<Vector2<i32>>>
    {
        PlayerTracker { steps: steps.into(), moves: 0 }
    }
}

pub fn player_controls(arg: RunArg, dir: Vector2<f32>)
{
    let (mut motion, positions, mut player, collision) = arg.fetch(|w| (w.write::<Motion>(), w.read::<Position>(), w.write::<Player>(), w.read::<Collision>()));

    for (motion, position, player) in (&mut motion, &positions, &mut player).join()
    {
        if motion.destination.is_some()
        {
            continue;
        }

        let pos = position.0;
        let dest = pos + dir;
        let dest_tile = dest.round_i32();

        let obstructed = {
            let mut obstructed = false;
            for (position, collision) in (&positions, &collision).join()
            {
                let pos_tile = position.0.round_i32();
                if pos_tile == dest_tile && collision == &Collision::Obstacle
                {
                    obstructed = true;
                    break;
                }
            }
            obstructed
        };

        if (pos.round_i32() == dest_tile) || obstructed
        {
            continue;
        }

        let dir = dest - pos;
        motion.destination = Some(Destination { position: dest, direction: dir });
        player.moves += 1;
    }
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
            let vel = destination.direction * motion.speed;
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

pub fn track_player(arg: RunArg)
{
    let (mut tracker, mut motion, position, player) = arg.fetch(|w| (w.write::<PlayerTracker>(), w.write::<Motion>(), w.read::<Position>(), w.read::<Player>()));

    // We're only going to acknowledge one player right now
    let player_pos: Vector2<i32>;
    let player_moves: u32;
    {
        let (player, position) = (&player, &position).join().next().expect("No player found");
        player_pos = position.0.round_i32();
        player_moves = player.moves;
    }

    for (tracker, motion, position) in (&mut tracker, &mut motion, &position).join()
    {
        if tracker.steps.back() != Some(&player_pos)
        {
            tracker.steps.push_back(player_pos);
        }

        if motion.destination.is_none() && tracker.moves < player_moves
        {
            if let Some(next_step) = tracker.steps.pop_front()
            {
                let pos = position.0;
                let dest = next_step.to_f32();
                let dir = dest - pos;
                motion.destination = Some(Destination { position: dest, direction: dir });
                tracker.moves += 1;
            }
        }
    }
}

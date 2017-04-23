use std::collections::VecDeque;

use specs::{RunArg, Join};

use systems::{Position};
use vectors::*;

#[derive(Default)]
pub struct Motion
{
    pub destination: Option<Destination>,
    pub speed: f32,
    pub delay_remaining: f32
}
component!(Motion);

impl Motion
{
    pub fn new(speed: f32) -> Self
    {
        Motion
        {
            destination: None,
            speed: speed,
            delay_remaining: 0.0
        }
    }

    pub fn move_from_to(&mut self, from_pos: Vector2<f32>, to_pos: Vector2<f32>)
    {
        self.destination = Some(Destination { position: to_pos, direction: to_pos - from_pos });
    }
}

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
    BlocksPush,
    Pushable,
    Obstacle
}
component!(Collision);

pub struct PlayerTracker
{
    pub steps: VecDeque<Vector2<i32>>,
    pub moves: u32,
    pub delay: f32
}
component!(PlayerTracker);

impl PlayerTracker
{
    pub fn new<T>(delay: f32, steps: T) -> Self
        where T: Into<VecDeque<Vector2<i32>>>
    {
        PlayerTracker { steps: steps.into(), moves: 0, delay: delay }
    }
}

pub fn player_controls(arg: RunArg, dir: Vector2<f32>)
{
    let (mut motion, positions, mut player, collisions) = arg.fetch(|w| (w.write::<Motion>(), w.read::<Position>(), w.write::<Player>(), w.read::<Collision>()));

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
            for (position, collision) in (&positions, &collisions).join()
            {
                let pos_tile = position.0.round_i32();
                if pos_tile == dest_tile
                {
                    if collision == &Collision::Obstacle
                    {
                        obstructed = true;
                        break;
                    }
                    else if collision == &Collision::Pushable
                    {
                        // Fucked up bad. Have to check for pushing things in like three places
                        let push_dest = dest_tile + dir.round_i32();
                        for (position, collision) in (&positions, &collisions).join()
                        {
                            if position.0.round_i32() == push_dest && collision != &Collision::Passable
                            {
                                obstructed = true;
                            }
                        }
                    }
                }
            }
            obstructed
        };

        if (pos.round_i32() == dest_tile) || obstructed
        {
            continue;
        }

        motion.move_from_to(pos, dest);
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
            if motion.delay_remaining > 0.0
            {
                motion.delay_remaining -= dt;
                continue;
            }

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
    let (mut tracker, mut motion, positions, player, collisions) = arg.fetch(|w| (w.write::<PlayerTracker>(), w.write::<Motion>(), w.read::<Position>(), w.read::<Player>(), w.read::<Collision>()));

    // We're only going to acknowledge one player right now
    let player_pos: Vector2<i32>;
    let player_moves: u32;
    {
        let (player, position) = (&player, &positions).join().next().expect("No player found");
        player_pos = position.0.round_i32();
        player_moves = player.moves;
    }

    // Spaghetti incoming
    for (tracker, motion, position) in (&mut tracker, &mut motion, &positions).join()
    {
        if tracker.steps.back() != Some(&player_pos)
        {
            tracker.steps.push_back(player_pos);
        }

        if motion.destination.is_none()
        {
            if tracker.moves < player_moves
            {
                let pos = position.0;
                let tile_pos = pos.round_i32();
                let mut looking_for_move = true;

                while looking_for_move
                {
                    let mut failed_to_move = false;

                    if let Some(next_step) = tracker.steps.pop_front()
                    {
                        let dir = next_step - tile_pos;
                        if dir.dot(dir) == 1
                        {
                            for (position, collision) in (&positions, &collisions).join()
                            {
                                if position.0.round_i32() == next_step && collision == &Collision::Pushable
                                {
                                    let push_dest = next_step + dir;
                                    for (position, collision) in (&positions, &collisions).join()
                                    {
                                        if position.0.round_i32() == push_dest && collision != &Collision::Passable
                                        {
                                            failed_to_move = true;
                                        }
                                    }
                                }
                            }
                        }
                        else
                        {
                            failed_to_move = true;
                        }

                        if !failed_to_move
                        {
                            let dest = next_step.to_f32();
                            motion.move_from_to(pos, dest);
                            tracker.moves += 1;
                            looking_for_move = false;
                        }
                    }
                    else
                    {
                        break;
                    }
                }
            }
            else if tracker.moves == player_moves
            {
                motion.delay_remaining = tracker.delay;
            }
        }
    }
}

pub fn push_stuff(arg: RunArg)
{
    let (entities, mut motion, position, collision) = arg.fetch(|w| (w.entities(), w.write::<Motion>(), w.read::<Position>(), w.read::<Collision>()));

    let mut pushes = Vec::new();

    for motion_a in (&motion).join()
    {
        if let Some(destination) = motion_a.destination
        {
            if motion_a.delay_remaining > 0.0
            {
                continue;
            }

            let dest_tile = destination.position.round_i32();

            for (entity, motion_b, position, collision) in (&entities, &motion, &position, &collision).join()
            {
                if collision != &Collision::Pushable || motion_b.destination.is_some()
                {
                    continue;
                }

                let pos = position.0.round_i32();
                if pos == dest_tile
                {
                    let push_dir = destination.direction;
                    let push_dest = position.0 + push_dir;
                    pushes.push((entity, push_dest, push_dir));
                }
            }
        }
    }

    for (entity, push_dest, push_dir) in pushes
    {
        let mut blocked = false;
        for (position, collision) in (&position, &collision).join()
        {
            let pos = position.0.round_i32();
            blocked |= pos == push_dest.round_i32() && collision != &Collision::Passable;
        }

        if !blocked
        {
            if let Some(motion) = motion.get_mut(entity)
            {
                motion.destination = Some(Destination { position: push_dest, direction: push_dir });
            }
        }
    }
}

use specs::{Component, World, Join, Gate};

use systems::{Position, Player};

pub struct Goal;
component!(Goal);

pub struct Hazard;
component!(Hazard);


fn player_touched_something_of_type<T: Component>(world: &World) -> bool
{
    let (position, player, thing) = (world.read::<Position>().pass(), world.read::<Player>().pass(), world.read::<T>().pass());
    for (player_position, _player) in (&position, &player).join()
    {
        let player_pos = player_position.0.round_i32();
        for (thing_position, _thing) in (&position, &thing).join()
        {
            let thing_pos = thing_position.0.round_i32();
            if player_pos == thing_pos
            {
                return true;
            }
        }
    }
    false
}

pub fn determine_gameover_from_hazard(world: &World) -> bool
{
    player_touched_something_of_type::<Hazard>(world)
}

pub fn determine_victory_from_goal(world: &World) -> bool
{
    player_touched_something_of_type::<Goal>(world)
}

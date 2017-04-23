use specs::{RunArg, Join};

use systems::{Position, Motion, Collision, Sprite};
use vectors::*;

pub struct Button(pub bool);
component!(Button);

pub struct ButtonGate(pub bool);
component!(ButtonGate);


pub fn check_button_presses(arg: RunArg)
{
    let (mut button, position, motion) = arg.fetch(|w| (w.write::<Button>(), w.read::<Position>(), w.read::<Motion>()));

    for (button_pos, button) in (&position, &mut button).join()
    {
        button.0 = false;
        let button_tile = button_pos.0.round_i32();

        for (obj_pos, _motion) in (&position, &motion).join()
        {
            let obj_tile = obj_pos.0.round_i32();
            if button_tile == obj_tile
            {
                button.0 = true;
                break;
            }
        }
    }
}

pub fn open_and_close_gates(arg: RunArg)
{
    let (mut collision, mut gate, button) = arg.fetch(|w| (w.write::<Collision>(), w.write::<ButtonGate>(), w.read::<Button>()));

    let mut button_pressed = false;
    for button in (&button).join()
    {
        button_pressed |= button.0;
    }

    for (collision, gate) in (&mut collision, &mut gate).join()
    {
        gate.0 = button_pressed;
        *collision = match button_pressed
        {
            true => Collision::Passable,
            false => Collision::Obstacle
        }
    }
}

pub fn update_gate_sprites(arg: RunArg)
{
    let (mut sprite, gate) = arg.fetch(|w| (w.write::<Sprite>(), w.read::<ButtonGate>()));

    for (sprite, gate) in (&mut sprite, &gate).join()
    {
        sprite.region = match gate.0
        {
            true => vec2(1, 4),
            false => vec2(0, 4)
        }
    }
}

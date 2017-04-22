use vectors::*;

pub const BG_LAYER: u32 = 0;
pub const OBJECT_LAYER: u32 = 1;
pub const ACTOR_LAYER: u32 = 2;
pub const FG_LAYER: u32 = 3;

pub struct Sprite
{
    pub region: Vector2<u32>,
    pub layer: u32
}
component!(Sprite);

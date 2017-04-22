pub struct Game
{
    pub resolution: (u32, u32),
    pub tile_size: u32
}

impl Game
{
    pub fn new(resolution: (u32, u32)) -> Self
    {
        Game
        {
            resolution: resolution,
            tile_size: 16
        }
    }
}

use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use find_folder::{Search};
use serde_yaml;

use vectors::*;

pub fn get_asset_path<P>(path: P) -> PathBuf
    where P: AsRef<Path>
{
    let assets = Search::ParentsThenKids(3, 3).for_folder("assets").expect("Could not find assets folder");
    let filepath = assets.join(path.as_ref());
    filepath
}

pub fn get_asset_string<P>(path: P) -> String
    where P: AsRef<Path>
{
    let path = get_asset_path(path);
    let mut file = File::open(&path).expect(&format!("Could not open file '{:?}'", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&format!("Failed to read file '{:?}'", path));
    contents
}

pub fn get_asset_bytes<P>(path: P) -> Vec<u8>
    where P: AsRef<Path>
{
    let path = get_asset_path(path);
    let mut file = File::open(&path).expect(&format!("Could not open file '{:?}'", path));
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect(&format!("Failed to read file '{:?}'", path));
    contents
}


pub fn load_levels<P>(path: P) -> Vec<Level>
    where P: AsRef<Path>
{
    let yaml = get_asset_string(path);
    let levelset: LevelSet = serde_yaml::from_str(&yaml).expect("Failed to parse levels");
    let mut levels = Vec::new();
    for leveldata in levelset.levels
    {
        let name = leveldata.name;
        let height = leveldata.tiles.len();
        let mut player_pos = None;
        let mut stalker_pos = None;
        let mut doors = Vec::new();
        let mut blocks = Vec::new();
        let mut width = 0;

        for (inv_y, row) in leveldata.tiles.iter().enumerate()
        {
            let y = height - inv_y - 1;
            for (x, code) in row.split(' ').enumerate()
            {
                width = max(width, x);
                let tilepos = vec2(x as f32, y as f32);
                match code
                {
                    "P" => player_pos = Some(tilepos),
                    "S" => stalker_pos = Some(tilepos),
                    "D" => doors.push(tilepos),
                    "=" => blocks.push((0, tilepos)),
                    ";" => blocks.push((1, tilepos)),
                    "." => (),
                    other => panic!(format!("Found unparsable character in level file: '{}'", other))
                }
            }
        }

        let midpoint = vec2(width as f32, height as f32) * 0.5 + vec2(0.0, -0.5);
        assert!(doors.len() > 0, "No doors in level!");

        let player_pos = player_pos.expect("No player position in level");
        let stalker_pos = stalker_pos.expect("No stalker position in level");

        let mut initial_stalker_path = Vec::new();
        {
            let disp = (player_pos - stalker_pos).round_i32();
            assert!(disp.components[0] == 0 || disp.components[1] == 0, "Stalker must be in a straight line from player");

            let steps = max(disp.components[0].abs(), disp.components[1].abs());
            let delta = vec2(disp.components[0] / steps, disp.components[1] / steps);
            for i in 1..steps
            {
                initial_stalker_path.push(stalker_pos.round_i32() + (delta * i));
            }
        }

        levels.push(Level
        {
            name: name,
            midpoint: midpoint,
            player_pos: player_pos,
            stalker_pos: stalker_pos,
            doors: doors,
            blocks: blocks,
            initial_stalker_path: initial_stalker_path
        });
    }

    levels
}

#[derive(Deserialize)]
struct LevelSet
{
    pub levels: Vec<LevelData>
}

#[derive(Deserialize)]
struct LevelData
{
    pub name: String,
    pub tiles: Vec<String>
}

pub struct Level
{
    pub name: String,
    pub midpoint: Vector2<f32>,
    pub player_pos: Vector2<f32>,
    pub stalker_pos: Vector2<f32>,
    pub doors: Vec<Vector2<f32>>,
    pub blocks: Vec<(u32, Vector2<f32>)>,
    pub initial_stalker_path: Vec<Vector2<i32>>
}

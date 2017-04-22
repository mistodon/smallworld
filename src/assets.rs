use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use find_folder::{Search};

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

use json::{object, JsonValue};

use super::{MineralType, TileType};
use crate::game::map::{tile::minerals::Mineral, Map, TerrainType};
pub const SETTINGS_PATH: &str = "./settings/tile/minerals/gen_params.json";

#[test]
fn read_file_example() {
    use std::fs;

    let data = fs::read_to_string(SETTINGS_PATH);
    let content = json::parse(&data.expect("!!!")).unwrap();
    println!("{}", content["ROCK"]["can replace"]);

    let mut map = Map::new();
    map.generate();
    println!("{:?}",map.gen_params);
}


impl MineralType {
    pub fn can_replace(self) -> Vec<TileType> {
        match self {
            MineralType::IRON => {
                vec![
                    TileType::TerrainType(TerrainType::AIR),
                    TileType::Mineral(MineralType::ROCK),
                ]
            }
            MineralType::GOLD => {
                vec![
                    // TileType::TerrainType(TerrainType::AIR),
                    TileType::Mineral(MineralType::IRON),
                    // TileType::Mineral(MineralType::ROCK),
                ]
            }
            MineralType::ROCK => {
                vec![
                    TileType::TerrainType(TerrainType::AIR),
                    TileType::TerrainType(TerrainType::WATER),
                    TileType::Mineral(MineralType::DIRT),
                ]
            }
            MineralType::DIRT => {
                vec![
                    TileType::TerrainType(TerrainType::AIR),
                    TileType::Mineral(MineralType::ROCK),
                ]
            }
            MineralType::MOSS => {
                vec![
                    TileType::TerrainType(TerrainType::WATER),
                    TileType::TerrainType(TerrainType::AIR),
                    TileType::Mineral(MineralType::DIRT),
                ]
            }
        }
    }
}

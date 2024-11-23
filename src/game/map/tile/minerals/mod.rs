pub(crate) mod gen_params;

use std::collections::VecDeque;

use coords::Coords;
use json::JsonValue;

use crate::game::{map::{Map, HEIGHT, WIDTH}, units::Unit};

use super::{Tile, TileType};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MineralType {
    IRON,
    GOLD,
    ROCK,
    MOSS,
    DIRT,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Mineral(pub MineralType);

/// # Mineral
/// 
/// A mineral is one of the 3 main TileType options
/// 
/// ```
/// let tile = Tile::new();
///  let iron = Mineral(MineralType::IRON);
/// 
/// tile.add(iron.to_tile_type());
/// 
/// ``` 

impl Mineral {
}

impl MineralType {
    pub fn to_str(self) -> &'static str{
        match self{
            MineralType::IRON => "IRON",
            MineralType::GOLD => "GOLD",
            MineralType::ROCK => "ROCK",
            MineralType::MOSS => "MOSS",
            MineralType::DIRT => "DIRT",
            _ => panic!(),
        }
    }
    pub fn to_tile_type(self) -> TileType {
        TileType::Mineral(self)
    }
   
    pub fn to_char(self) -> String {
        match self {
            MineralType::IRON => "I".to_string(),
            MineralType::GOLD => "G".to_string(),
            MineralType::ROCK => "R".to_string(),
            MineralType::MOSS => "M".to_string(),
            MineralType::DIRT => "D".to_string(),
        }
    }
    pub fn color(self) -> u32 {
        match self {
            MineralType::IRON => 0xa89172ff,
            MineralType::GOLD => 0xffff1cff,
            MineralType::ROCK => 0x303030ff,
            MineralType::MOSS => 0x11aa1155,
            MineralType::DIRT => 0x140c07ff,
        }
    }

    pub fn find_closest(&self, tile: Tile, map: &Map, unit: Unit) -> Option<Coords> {
        let start = unit.coords;
        let mut visited = vec![vec![false; WIDTH]; HEIGHT];
        let mut queue = VecDeque::new();

        queue.push_back((start, 0));
        visited[start.x() as usize][start.y() as usize] = true;

        while let Some((coords, distance)) = queue.pop_front() {
            // Check if the current tile is a mineral
            if Ok((tile, coords)) == map.get_tile_from_coords(coords) {
                return Some(coords);
            }

            // Add neighboring tiles to the queue
            for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let neighbor = Coords(coords.x() + dir.0, coords.y() + dir.1);

                // Ensure the neighbor is within bounds and not yet visited
                if map.check_data(neighbor.x() as usize, neighbor.y() as usize)
                    && !visited[neighbor.x() as usize][neighbor.y() as usize]
                {
                    visited[neighbor.x() as usize][neighbor.y() as usize] = true;
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }
        None // No mineral found
    }
}

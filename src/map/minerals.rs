use std::collections::VecDeque;

use coords::Coords;

use crate::{map::terrain::TerrainType, units::Unit};

use super::{Map, Tile, TileType, HEIGHT, WIDTH};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MineralType {
    IRON,
    GOLD,
    ROCK,
    MOSS,
    DIRT,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Mineral(pub MineralType);

impl Mineral {
    pub fn new(mineral_type: MineralType) -> Mineral {
        Mineral(mineral_type)
    }
}

impl MineralType {
    pub fn to_tile_type(self) -> TileType {
        TileType::Mineral(self)
    }
    pub fn can_replace(self) -> Vec<TileType> {
        match self {
            MineralType::IRON => {
                vec![
                    TileType::TerrainType(TerrainType::AIR),
                    TileType::Mineral(MineralType::DIRT),
                ]
            }
            MineralType::GOLD => {
                vec![
                    TileType::TerrainType(TerrainType::AIR),
                    TileType::Mineral(MineralType::IRON),
                    TileType::Mineral(MineralType::ROCK),
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
                vec![TileType::TerrainType(TerrainType::AIR)]
            }
            MineralType::MOSS => {
                vec![
                    TileType::TerrainType(TerrainType::WATER),
                    TileType::TerrainType(TerrainType::AIR),
                ]
            }
        }
    }
    pub fn birth_limit(self) -> usize {
        match self {
            MineralType::IRON => 2,
            MineralType::GOLD => 8,
            MineralType::ROCK => 5,
            MineralType::MOSS => 6,
            MineralType::DIRT => 5,
        }
    }
    pub fn death_limit(self) -> usize {
        match self {
            MineralType::IRON => 3,
            MineralType::GOLD => 3,
            MineralType::ROCK => 3,
            MineralType::MOSS => 3,
            MineralType::DIRT => 3,
        }
    }
    pub fn iterations(self) -> usize {
        match self {
            MineralType::IRON => 0,
            MineralType::GOLD => 0,
            MineralType::ROCK => 0,
            MineralType::MOSS => 0,
            MineralType::DIRT => 0,
        }
    }
    pub fn perlin_scale(self) -> f64 {
        match self {
            MineralType::IRON => 0.09,
            MineralType::GOLD => 0.03,
            MineralType::ROCK => 0.05,
            MineralType::MOSS => 0.1,
            MineralType::DIRT => 0.045,
        }
    }
    pub fn perlin_threshold(self) -> f64 {
        match self {
            MineralType::IRON => 0.01,
            MineralType::GOLD => 0.9,
            MineralType::ROCK => 0.05,
            MineralType::MOSS => 0.1,
            MineralType::DIRT => 0.37,
        }
    }
    pub fn occurence(self) -> f64 {
        match self {
            MineralType::IRON => 0.8,
            MineralType::GOLD => -1.0,
            MineralType::ROCK => 0.5,
            MineralType::MOSS => 0.5,
            MineralType::DIRT => 1.0,
        }
    }
    pub fn max_air_exposure(self) -> usize {
        match self {
            MineralType::IRON => 2,
            MineralType::GOLD => 8,
            MineralType::ROCK => 5,
            MineralType::MOSS => 8,
            MineralType::DIRT => 5,
        }
    }

    pub fn to_ascii(self) -> String {
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

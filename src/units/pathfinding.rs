use std::collections::VecDeque;

use coords::Coords;
use pathfinding::prelude::astar;

use super::{ActionType, Unit};
use crate::map::{
    minerals::{Mineral, MineralType},
    terrain::TerrainType,
    Map, Tile, TileType, HEIGHT, WIDTH,
};

impl Tile {
    /// AIR
    /// WATER
    /// Any Building
    pub fn is_walkable(self) -> bool {
        match self {
            Tile(Some(TerrainType::AIR), None, _) => true,
            Tile(Some(TerrainType::WATER), None, _) => true,
            Tile(_, Some(Mineral(MineralType::MOSS)), _) => true,
            Tile(_, _, Some(_)) => true,
            _ => false,
        }
    }
    /// Any Mineral
    pub fn is_diggable(self) -> bool {
        match self {
            Tile(_, Some(Mineral(MineralType::DIRT)), _) => true,
            Tile(_, Some(Mineral(MineralType::ROCK)), _) => true,
            Tile(_, Some(Mineral(MineralType::IRON)), _) => true,
            Tile(_, Some(Mineral(MineralType::GOLD)), _) => true,
            Tile(_, Some(Mineral(MineralType::MOSS)), _) => true,

            _ => false,
        }
    }
 
}

/// APPLY MOVEMENT COST RULES
/// returns i32 depending on unit's race, path preference etc
fn get_movement_cost(
    unit: Unit,
    is_diagonal: bool,
    terrain: &Map,
    action: Option<ActionType>,
    coords: Coords,
) -> i32 {
    // return unit.race.diagonal_cost(is_diagonal);
    unit.race.diagonal_cost(is_diagonal)
        + if let Ok(tile) = terrain.get_tile_from_coords(coords) {
            match (tile, action, unit.job) {
                // Ghost pathfinding
                (_, None, _) => {
                    if tile.is_diggable() {
                        0
                    } else if tile.is_walkable() {
                        0
                    } else {
                        10
                    }
                }
                // Pathfinding through minerals
                (_, Some(ActionType::DIG), _) => {
                    if tile.is_diggable() {
                        0
                    } else {
                        10
                    }
                }
                // Normal pathfinding (through empty tiles)
                (_, Some(ActionType::MOVE), _) => {
                    if tile.is_walkable() {
                        0
                    } else {
                        10
                    }
                }
                (_, _, _) => 1,
            }
        } else {
            // Invalid tile
            100
        }
}

impl Map {
    /// True if terrain.get_data(x, y) is diggable (see TileType.is_diggable)
    pub fn is_diggable(&mut self, x: usize, y: usize) -> bool {
        if let Ok(tile) = self.get_tile(x, y) {
            tile.is_diggable()
        } else {
            false
        }
    }
    pub fn find_closest(&self, coords: Coords, tile_type: &TileType) -> Option<Coords> {
        let start = coords;
        let mut visited = vec![vec![false; WIDTH]; HEIGHT];
        let mut queue = VecDeque::new();

        queue.push_back((start, 0));
        visited[start.x() as usize][start.y() as usize] = true;
        println!("--- Looking for : {:?} ", tile_type);

        while let Some((coords, distance)) = queue.pop_front() {
            // Add neighboring tiles to the queue
            for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let neighbor = coords + Coords(dir.0, dir.1);

                // Check if the current tile is the Tile we are looking for
                let curr_tile = self.get_tile_from_coords(neighbor);
                println!("{:?}", curr_tile);
                if curr_tile.is_ok() && curr_tile.unwrap().has(*tile_type) {
                    println!("{:?} found at {:?}", tile_type, curr_tile);
                    return Some(neighbor);
                }

                // Ensure the neighbor is within bounds and not yet visited
                if self.check_data(neighbor.x() as usize, neighbor.y() as usize)
                    && !visited[neighbor.x() as usize][neighbor.y() as usize]
                {
                    visited[neighbor.x() as usize][neighbor.y() as usize] = true;
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }

        None // No tile found
    }
}

impl Unit {
    ///
    /// TO DO : Stop pathfinding  when a limit is reached to improve performance
    ///
    /// Find shortest path depending on race rules
    /// - see get_movement_cost()
    pub fn find_path(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
        mut terrain: Map,
        action: Option<ActionType>,
    ) -> Option<(Vec<(usize, usize)>, i32)> {
        let (path, cost) = astar(
            &start,
            |&(x, y)| {
                // Définir les voisins cardinaux et diagonaux
                let directions = vec![
                    (x + 1, y, false),
                    (x.wrapping_sub(1), y, false),
                    (x, y + 1, false),
                    (x, y.wrapping_sub(1), false),
                    (x + 1, y + 1, true),
                    (x + 1, y.wrapping_sub(1), true),
                    (x.wrapping_sub(1), y + 1, true),
                    (x.wrapping_sub(1), y.wrapping_sub(1), true),
                ];
                directions
                    .into_iter()
                    .filter_map(|(nx, ny, is_diagonal)| match action {
                        Some(action) => {
                            // Avoir empty tiles entirely
                            if terrain.check_data(nx, ny) {
                                Some((
                                    (nx, ny),
                                    // Get cost based on unit preferences/job
                                    get_movement_cost(
                                        self.clone(),
                                        is_diagonal,
                                        &terrain,
                                        Some(action),
                                        Coords(nx as i32, ny as i32),
                                    ),
                                ))
                            } else {
                                None
                            }
                        }
                        _ => Some(((nx, ny), 0)),
                    })
                    .collect::<Vec<_>>()
            },
            |&(x, y)| {
                let dx = (x as isize - goal.0 as isize).abs();
                let dy = (y as isize - goal.1 as isize).abs();
                ((dx.pow(2) + dy.pow(2)) as f64).sqrt() as i32
            },
            |&pos| pos == goal,
        )?;

        let filtered_path = match action {
            None | Some(ActionType::MOVE) => {
                let mut last_walkable_path = Vec::new();
                // Only keep walkable tiles
                for &(x, y) in path.iter() {
                    if terrain.is_walkable(Coords(x.try_into().unwrap(), y.try_into().unwrap())) {
                        last_walkable_path.push((x, y));
                    } else {
                        // Stop when a non walkable tile is reached
                        break;
                    }
                }
                last_walkable_path
            }
            Some(ActionType::DIG) => {
                let mut first_diggable_path = Vec::new();
                for &(x, y) in path.iter() {
                    // Only keep diggable tiles
                    if terrain.is_diggable(x, y) {
                        first_diggable_path.push((x, y));
                        // Keep going until tileType changes
                        break;
                    }
                }
                first_diggable_path
            }
            _ => return None,
        };

        Some((filtered_path, cost))
    }
}

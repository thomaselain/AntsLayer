use pathfinding::prelude::astar;

use crate::{
    buildings::Building,
    coords::Coords,
    minerals::{Mineral, MineralType},
    terrain::{Terrain, TerrainType, Tile, TileType},
};

use super::{ActionType, Unit};

impl Tile {
    /// AIR
    /// WATER
    /// Any Building
    pub fn is_walkable(self) -> bool {
        match self {
            Tile(Some(TerrainType::AIR), None, _) => true,
            Tile(Some(TerrainType::WATER), None, _) => true,
            Tile(_, Some(Mineral(MineralType::MOSS)), _) => true,
            Tile(_, _, Some(Building { .. })) => true,
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
    terrain: &Terrain,
    action: Option<ActionType>,
    coords: Coords,
) -> i32 {
    // return unit.race.diagonal_cost(is_diagonal);
    unit.race.diagonal_cost(is_diagonal)
        + if let Some(tile) = terrain.get_tiles_from_coords(coords) {
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

impl Terrain {
    /// True if in terrain range
    pub fn check_data(&self, x: usize, y: usize) -> bool {
        if x < self.data.len() && y < self.data[x].len() {
            match self.data[x][y] {
                Tile(None, None, None) => false,
                _ => true,
            }
        } else {
            false
        }
    }

    /// return Some(self.data[x][y]) with overflow checks
    pub fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if self.check_data(x, y) {
            Some(self.data[x][y])
        } else {
            None
        }
    }

    /// return Some(self.data[x][y]) with overflow checks
    pub fn get_tiles_from_coords(&self, coords: Coords) -> Option<Tile> {
        self.get_tile(coords.x as usize, coords.y as usize)
    }

    /// True if terrain.get_data(x, y) is walkable (see TileType.is_walkable)
    pub fn is_walkable(&mut self, x: usize, y: usize) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            tile.is_walkable()
        } else {
            false
        }
    }

    /// True if terrain.get_data(x, y) is walkable (see TileType.is_diggable)
    pub fn is_diggable(&mut self, x: usize, y: usize) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            tile.is_diggable()
        } else {
            false
        }
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
        mut terrain: Terrain,
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
                                        Coords {
                                            x: nx as i32,
                                            y: ny as i32,
                                        },
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
                    if terrain.is_walkable(x, y) {
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
                    // None if self is not a miner
                    let target = self.job.get_miner_target();

                    // Only keep diggable tiles
                    if terrain.is_diggable(x, y) && !target.is_none() {
                        self.job.get_action(&terrain, self).0;
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

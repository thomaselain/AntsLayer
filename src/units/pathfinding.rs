use pathfinding::prelude::astar;
use std::{collections::VecDeque, ops::Mul};

use crate::{
    buildings::BuildingType,
    coords::Coords,
    terrain::{self, MineralType, Terrain, TileType},
};

use super::{ActionType, JobType, RaceType, Unit};

fn get_movement_cost(
    unit: Unit,
    is_diagonal: bool,
    terrain: &Terrain,
    action: Option<ActionType>,
    coords: Coords,
) -> i32 {
    // return unit.race.diagonal_cost(is_diagonal);
    unit.race.diagonal_cost(is_diagonal)+
    if let Some(tile) = terrain.get_data_from_coords(coords) {
        match (tile, action, unit.job) {
            (_, None, _) => {
                if tile.is_diggable() {
                    0
                } else if tile.is_walkable() {
                    unit.race.diagonal_cost(is_diagonal)
                } else {
                    0
                }
            } // Ghost pathfinding
            (_, Some(ActionType::DIG), _) => {
                if tile.is_diggable() {
                    0
                } else {
                    10
                }
            }
            (_, Some(ActionType::MOVE), _) => {
                if tile.is_walkable() {
                    0
                } else {
                    10
                }
            }
            (_, _, _) => {
                1
            }
        }
    } else {
        // Invalid tile
        100
    }
}

impl Terrain {
    pub fn get_data(&self, x: usize, y: usize) -> Option<TileType> {
        if self.check_data(x, y) {
            Some(self.data[x][y])
        } else {
            None
        }
    }

    pub fn get_data_from_coords(&self, coords: Coords) -> Option<TileType> {
        self.get_data(coords.x as usize, coords.y as usize)
    }

    pub fn check_data(&self, x: usize, y: usize) -> bool {
        if x < self.data.len() && y < self.data[x].len() {
            true
        } else {
            false
        }
    }
    pub fn count_same_neighbors(&mut self, x: usize, y: usize, tile_type: TileType) -> usize {
        let mut count = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if self.get_data(nx as usize, ny as usize) == Some(tile_type) {
                    count += 1;
                }
            }
        }
        count
    }
}
impl Unit {
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
                            if terrain.check_data(nx, ny) {
                                Some((
                                    (nx, ny),
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
                // Filtrer pour obtenir les cases walkable uniquement
                let mut last_walkable_path = Vec::new();
                for &(x, y) in path.iter() {
                    if terrain.is_walkable(x, y) {
                        last_walkable_path.push((x, y));
                    } else {
                        break;
                    }
                }
                last_walkable_path
            }
            Some(ActionType::DIG) => {
                let mut first_diggable_path = Vec::new();
                for &(x, y) in path.iter() {
                    if terrain.is_diggable(x, y) {
                        first_diggable_path.push((x, y));
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

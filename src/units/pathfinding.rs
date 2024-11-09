use pathfinding::prelude::astar;

use crate::terrain::{MineralType, Terrain, TileType};

use super::{ActionType, RaceType, Unit};

impl Terrain {
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        match self.get_data(x, y) {
            Some(TileType::AIR) => true,
            Some(TileType::WATER) => true,
            _ => false,
        }
    }

    pub fn is_diggable(&self, x: usize, y: usize) -> bool {
        match self.get_data(x, y) {
            Some(TileType::Mineral(MineralType::ROCK)) => true,
            Some(TileType::Mineral(MineralType::DIRT)) => true,
            _ => false,
        }
    }
    pub fn get_data(&self, x: usize, y: usize) -> Option<TileType> {
        if self.check_data(x, y) {
            Some(self.data[x][y])
        } else {
            None
        }
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
    fn get_movement_cost(&self, is_diagonal: bool, action: Option<ActionType>) -> i32 {
        match self.race {
            RaceType::ANT => {
                if let Some(ActionType::DIG) = action {
                    return 10;
                }
                if is_diagonal {
                    1
                } else {
                    1
                }
            }
            RaceType::HUMAN => {
                if let Some(ActionType::DIG) = action {
                    return 10;
                }
                if is_diagonal {
                    5
                } else {
                    1
                }
            }
            RaceType::ALIEN => {
                if let Some(ActionType::DIG) = action {
                    return 10;
                }
                if is_diagonal {
                    1
                } else {
                    5
                }
            }
        }
    }

    pub fn find_path(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
        terrain: Terrain,
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
                        Some(ActionType::MOVE) => {
                            if terrain.is_walkable(nx, ny) {
                                Some((
                                    (nx, ny),
                                    self.get_movement_cost(is_diagonal, Some(ActionType::MOVE)),
                                ))
                            } else {
                                None
                            }
                        }
                        Some(ActionType::DIG) => {
                            if terrain.is_diggable(nx, ny) {
                                Some((
                                    (nx, ny),
                                    self.get_movement_cost(is_diagonal, Some(ActionType::DIG)),
                                ))
                            } else {
                                None
                            }
                        }
                        None => {
                            if terrain.is_diggable(nx, ny) || terrain.is_walkable(nx, ny) {
                                Some(((nx, ny), self.get_movement_cost(is_diagonal, None)))
                            } else {
                                None
                            }
                        }
                        _ => None,
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

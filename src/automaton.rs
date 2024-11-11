extern crate automata;

use crate::{
    terrain::{Terrain, TileType},
    window::{HEIGHT, WIDTH},
};

#[derive(Clone)]
// Structure Automaton associée à chaque mineral
pub struct Automaton {
    /// List of TileType's that this mineral can replace
    pub can_replace: Vec<TileType>,
    /// Maximum neighbors of TileType::AIR for the tile to survive
    pub max_air_exposure: usize,
    /// Minimum neighbors for a tile to be created
    pub birth_limit: usize,
    /// Maximum neighbors for a tile to survive
    pub death_limit: usize,
    /// How many times we loop through the automaton
    pub iterations: usize,
    /// Perlin noise zoom level
    pub perlin_scale: f64,
    /// Threshold for Perlin noise
    /// Between 0.0 and 1.0
    /// represents how "high" the noise can go
    pub perlin_threshold: f64,
    /// Percentage we keep from generated Perlin (from -1.0 to 1.0)
    /// Set to negative to reverse Perlin generation (you then probably must tweak some stuff in automaton settings)
    pub occurence: f64,
}

impl Automaton {
    /// Automaton rules
    pub fn apply_rules(&self, terrain: &mut Terrain, mineral_type: TileType) {
        for _ in 0..self.iterations {
            let mut new_data = terrain.data.clone();
            for x in 1..(WIDTH as usize - 1) {
                for y in 1..(HEIGHT as usize - 1) {
                    let mut can_replace: bool = false;
                    for _ in &self.can_replace {
                        if terrain.get_data(x, y) == Some(mineral_type) {
                            can_replace = true;
                        }
                    }

                    let count_same = terrain.count_same_neighbors(x, y, mineral_type);
                    let count_air = terrain.count_same_neighbors(x, y, TileType::AIR);
                    if can_replace {
                        if terrain.get_data(x, y) == Some(mineral_type) && can_replace {
                            if count_same < self.death_limit {
                                new_data[x][y] = TileType::AIR;
                            }
                        } else if terrain.get_data(x, y) == Some(TileType::AIR)
                            && count_air <= self.max_air_exposure
                        {
                            // loop self.priority_list to find out if we replace
                            if count_same > self.birth_limit {
                                new_data[x][y] = mineral_type;
                            }
                        }
                    }
                }
            }
            terrain.data = new_data;
        }
    }
}

extern crate automata;

use crate::{
    terrain::{Terrain, TileType},
    window::{HEIGHT, WIDTH},
};

#[derive(Copy, Clone)]
// Structure Automaton associée à chaque mineral
pub struct Automaton {
    pub birth_limit: usize,
    pub death_limit: usize,
    pub iterations: usize,
    pub perlin_scale: f64,
    pub perlin_threshold: f64,
    pub occurence: f64,
}

impl Automaton {
    pub fn new(
        birth_limit: usize,
        death_limit: usize,
        iterations: usize,
        perlin_scale: f64,
        perlin_threshold: f64,
        occurence: f64,
    ) -> Automaton {
        Automaton {
            birth_limit,
            death_limit,
            iterations,
            perlin_scale,
            perlin_threshold,
            occurence,
        }
    }


    // Exemple d'application d'une règle spécifique
    pub fn apply_rules(&self, terrain: &mut Terrain, mineral_type: TileType) {
        for _ in 0..self.iterations {
            let mut new_data = terrain.data.clone();

            for x in 1..(WIDTH as usize - 1) {
                for y in 1..(HEIGHT as usize - 1) {
                    let count = terrain.count_same_neighbors(x, y, mineral_type);
                    if terrain.get_data(x, y) == Some(mineral_type) {
                        if count < self.death_limit {
                            new_data[x][y] = TileType::AIR;
                        }
                    } else if terrain.get_data(x, y) == Some(TileType::AIR) {
                        if count > self.birth_limit {
                            new_data[x][y] = mineral_type;
                        }
                    }
                }
            }

            terrain.data = new_data;
        }
    }
}

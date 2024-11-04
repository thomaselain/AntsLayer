extern crate noise;
extern crate sdl2;


use noise::{NoiseFn, Perlin};
use rand::{self, Rng};

use crate::{
    automaton::Automaton,
};
pub const HEIGHT: usize = 300;
pub const WIDTH: usize = 300;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MineralType {
    IRON,
    GOLD,
    ROCK,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Mineral(MineralType),
    AIR,
    WATER,
}

#[derive(Copy, Clone)]
pub struct Mineral {
    pub r#type: TileType,
    pub automaton: Automaton,
    pub color: u32,
}

#[derive(Clone)]
pub struct Terrain {
    pub minerals: Vec<Mineral>,
    pub data: Vec<Vec<TileType>>,
}

impl Terrain {
    pub fn new() -> Terrain {
        let tiles: Vec<Vec<TileType>> = vec![vec![TileType::AIR; WIDTH as usize]; HEIGHT as usize];


        Terrain {
            data: tiles,
            minerals: vec![
                Mineral {
                    r#type: TileType::Mineral(MineralType::GOLD),
                    color: 0xffff1cff,
                    automaton: Automaton::new(4, 6, 3, 0.05, 0.045, 0.95),
                },
                Mineral {
                    r#type: TileType::Mineral(MineralType::IRON),
                    color: 0xAAAAAAff,
                    automaton: Automaton::new(4, 5, 4, 0.05, 0.075, 1.0),
                },
                Mineral {
                    r#type: TileType::Mineral(MineralType::ROCK),
                    color: 0x303030FF,
                    automaton: Automaton::new(4, 4, 5, 0.035, 0.35, 1.0),
                },
            ],
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

    pub fn generate_caves(&mut self, mineral: &Mineral) {
        let mut rng = rand::thread_rng();
        let noise: Perlin = Perlin::new(rng.gen());
        println!("Color = {:#X}", mineral.color); // Ajout pour le debug

        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                if self.check_data(x, y) {
                    let noise_value = noise.get([
                        x as f64 * mineral.automaton.perlin_scale,
                        y as f64 * mineral.automaton.perlin_scale,
                    ]);
                    if noise_value.abs()
                        < mineral.automaton.perlin_threshold * mineral.automaton.occurence
                    {
                        self.data[x][y] = mineral.r#type;
                    }
                }
            }
        }
        // Application des règles de l'automate cellulaire
        mineral.automaton.apply_rules(self, mineral.r#type);
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

    fn clear_tiles(&mut self) {
        self.data = vec![vec![TileType::AIR; WIDTH as usize]; HEIGHT as usize];
    }
    pub fn generate(&mut self) {
        self.minerals.sort_by(|b, a| {
            b.automaton
                .occurence
                .partial_cmp(&a.automaton.occurence)
                .unwrap()
        });

        let minerals_copy: Vec<Mineral> = self.minerals.clone();

        self.clear_tiles();
        for m in minerals_copy {
            self.generate_caves(&m);
        }
    }
}

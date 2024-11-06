extern crate noise;
extern crate sdl2;

use noise::{NoiseFn, Perlin};
use rand::{self, Rng};

use crate::{
    automaton::Automaton,
    buildings::{Building, BuildingType},
    coords::Coords,
    units::RaceType,
};

pub const HEIGHT: usize = 300;
pub const WIDTH: usize = 300;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MineralType {
    IRON,
    GOLD,
    ROCK,
    DIRT,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Building(BuildingType),
    Mineral(MineralType),
    AIR,
    WATER,
}

#[derive(Clone)]
pub struct Mineral {
    pub r#type: TileType,
    pub automaton: Automaton,
    pub color: u32,
}

#[derive(Clone)]
pub struct Terrain {
    pub buildings: Vec<(RaceType, Building)>,
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
                    r#type: TileType::Mineral(MineralType::ROCK),
                    color: 0x303030FF,
                    automaton: Automaton {
                        can_replace: vec![TileType::AIR, TileType::Mineral(MineralType::DIRT)],
                        birth_limit: 5,
                        death_limit: 5,
                        iterations: 0,
                        perlin_scale: 0.05,
                        perlin_threshold: 0.05,
                        occurence: 1.0,
                        max_air_exposure: 5,
                    },
                },
                Mineral {
                    r#type: TileType::Mineral(MineralType::DIRT),
                    color: 0x2b180cff,
                    automaton: Automaton {
                        can_replace: vec![
                            //TileType::Mineral(MineralType::ROCK),
                            // TileType::Mineral(MineralType::DIRT),
                            TileType::AIR,
                        ],
                        birth_limit: 5,
                        death_limit: 6,
                        iterations: 0,
                        perlin_scale: 0.045,
                        perlin_threshold: 0.37,
                        occurence: 1.0,
                        max_air_exposure: 5,
                    },
                },
                Mineral {
                    r#type: TileType::Mineral(MineralType::IRON),
                    color: 0xAAAAAAff,
                    automaton: Automaton {
                        can_replace: vec![
                            TileType::Mineral(MineralType::ROCK),
                            // TileType::Mineral(MineralType::DIRT),
                            //TileType::AIR,
                        ],
                        birth_limit: 2,
                        death_limit: 5,
                        iterations: 7,
                        perlin_scale: 0.09,
                        perlin_threshold: 0.01,
                        occurence: 1.0,
                        max_air_exposure: 2,
                    },
                },
                Mineral {
                    r#type: TileType::Mineral(MineralType::GOLD),
                    color: 0xffff1cff,
                    automaton: Automaton {
                        can_replace: vec![
                            TileType::Mineral(MineralType::ROCK),
                            // TileType::Mineral(MineralType::DIRT),
                        ],
                        birth_limit: 8,
                        death_limit: 4,
                        iterations: 5,
                        perlin_scale: 0.03,
                        perlin_threshold: 0.9,
                        occurence: -1.0,
                        max_air_exposure: 8,
                    },
                },
            ],
            buildings: vec![
                (
                    RaceType::ANT,
                    Building {
                        hp: 100,
                        coords: Coords { x: 10, y: 10 },
                        building_type: BuildingType::Hearth,
                        race: RaceType::ANT,
                    },
                ),
                (
                    RaceType::HUMAN,
                    Building {
                        hp: 100,
                        coords: Coords {
                            x: WIDTH as i32 / 2,
                            y: HEIGHT as i32 / 2,
                        },
                        building_type: BuildingType::Hearth,
                        race: RaceType::HUMAN,
                    },
                ),
                (
                    RaceType::ALIEN,
                    Building {
                        hp: 100,
                        coords: Coords {
                            x: WIDTH as i32 - 10,
                            y: HEIGHT as i32 - 10,
                        },
                        building_type: BuildingType::Hearth,
                        race: RaceType::ALIEN,
                    },
                ),
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
        if mineral.automaton.occurence == 0.0 {
            return;
        } // Set to 0.0 to skip (for testing)

        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                let tile = self.get_data(x, y);
                let mut can_replace: bool = false;

                for t in &mineral.automaton.can_replace {
                    if tile.unwrap() == *t {
                        can_replace = true;
                    }
                }
                let noise_value = noise.get([
                    x as f64 * mineral.automaton.perlin_scale,
                    y as f64 * mineral.automaton.perlin_scale,
                ]);
                if can_replace {
                    match tile {
                        Some(TileType::Mineral(_)) => {
                            if noise_value
                                < mineral.automaton.perlin_threshold * mineral.automaton.occurence
                            {
                                self.data[x][y] = mineral.r#type;
                            }
                        }
                        Some(TileType::AIR) => {
                            if noise_value
                                < mineral.automaton.perlin_threshold * mineral.automaton.occurence
                            {
                                self.data[x][y] = mineral.r#type;
                            }
                        }
                        Some(TileType::WATER) => {}
                        Some(TileType::Building(_)) => {}
                        None => todo!(),
                    }
                }
            }
        }
        // Application des règles de l'automate cellulaire
        mineral.automaton.apply_rules(self, mineral.r#type);
        //  mineral.automaton.apply_rules(self, TileType::AIR);
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
        /*  self.minerals.sort_by(|b, a| {
                    b.automaton
                        .occurence
                        .partial_cmp(&a.automaton.occurence)
                        .unwrap()
                });
        */
        let minerals_copy: Vec<Mineral> = self.minerals.clone();

        self.clear_tiles();
        for m in minerals_copy {
            self.generate_caves(&m);
        }
        for b in self.buildings.clone() {
            self.data[b.1.coords.x as usize][b.1.coords.y as usize] =
                TileType::Building(b.1.building_type);
            self.dig_home(b.1.coords, 5);
            self.data[b.1.coords.x as usize + 3][b.1.coords.y as usize] =
                TileType::Building(BuildingType::Stockpile);
        }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if let Some(tile) = self.get_data(x, y) {
            matches!(tile, TileType::AIR | TileType::Building(_))
        } else {
            false
        }
    }
    pub fn dig_home(&mut self, center: Coords, radius: u32) {
        let (cx, cy) = (center.x as i32, center.y as i32);
        let radius_squared = (radius * radius) as i32;

        for y in (cy - radius as i32)..=(cy + radius as i32) {
            for x in (cx - radius as i32)..=(cx + radius as i32) {
                let dx = x - cx;
                let dy = y - cy;
                if !(x == center.x && y == center.y) {
                    if dx * dx + dy * dy <= radius_squared {
                        if let Some(_) = self.get_data(x as usize, y as usize) {
                            self.data[x as usize][y as usize] = TileType::AIR; // Dig
                        }
                    }
                }
            }
        }
    }
}

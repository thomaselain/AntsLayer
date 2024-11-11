extern crate noise;
extern crate sdl2;

use std::collections::VecDeque;

use crate::{
    automaton::Automaton,
    buildings::{self, Building, BuildingType},
    coords::Coords,
    units::{RaceType, Unit},
};
use noise::{NoiseFn, Perlin};
use rand::{self, Rng};

pub const HEIGHT: usize = 500;
pub const WIDTH: usize = HEIGHT;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MineralType {
    IRON,
    GOLD,
    ROCK,
    DIRT,
}

impl MineralType {
    pub fn find_closest(
        &self,
        tile_type: TileType,
        terrain: &Terrain,
        unit: Unit,
    ) -> Option<Coords> {
        let start = unit.coords;
        let mut visited = vec![vec![false; WIDTH]; HEIGHT];
        let mut queue = VecDeque::new();

        queue.push_back((start, 0));
        visited[start.x as usize][start.y as usize] = true;

        while let Some((coords, distance)) = queue.pop_front() {
            // Check if the current tile is a mineral
            if Some(tile_type) == terrain.get_data_from_coords(coords) {
                return Some(coords);
            }

            // Add neighboring tiles to the queue
            for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let neighbor = Coords {
                    x: coords.x + dir.0,
                    y: coords.y + dir.1,
                };

                // Ensure the neighbor is within bounds and not yet visited
                if terrain.check_data(neighbor.x as usize, neighbor.y as usize)
                    && !visited[neighbor.x as usize][neighbor.y as usize]
                {
                    visited[neighbor.x as usize][neighbor.y as usize] = true;
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }
        None // No mineral found
    }
    pub fn _is_collectable(self) -> bool {
        match self {
            Self::ROCK => false,
            Self::IRON => true,
            Self::GOLD => true,
            Self::DIRT => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Building(BuildingType),
    Mineral(MineralType),
    AIR,
    WATER,
}
impl TileType {
    /// AIR
    /// WATER
    /// Any Building
    pub fn is_walkable(self) -> bool {
        match self {
            TileType::AIR => true,
            TileType::WATER => true,
            TileType::Building(_) => true,
            _ => false,
        }
    }
    /// Any Mineral
    pub fn is_diggable(self) -> bool {
        match self {
            TileType::Mineral(MineralType::IRON) => true,
            TileType::Mineral(MineralType::GOLD) => true,
            TileType::Mineral(MineralType::ROCK) => true,
            TileType::Mineral(MineralType::DIRT) => false,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Mineral {
    pub r#type: TileType,
    pub automaton: Automaton,
    pub color: u32,
}

#[derive(Clone)]
/// Main structure for terrain manipulation and data storage
/// Each mineral in minerals have an automaton with set rules for generation in generate_caves()
/// data contains all map data
pub struct Terrain {
    pub buildings: Vec<Building>,
    pub minerals: Vec<Mineral>,
    pub data: Vec<Vec<TileType>>,
}

impl Terrain {
    /// True if terrain.get_data(x, y) is walkable (see TileType.is_walkable)
    pub fn is_walkable(&mut self, x: usize, y: usize) -> bool {
        if let Some(tile) = self.get_data(x, y) {
            tile.is_walkable()
        } else {
            false
        }
    }

    /// True if terrain.get_data(x, y) is walkable (see TileType.is_diggable)
    pub fn is_diggable(&mut self, x: usize, y: usize) -> bool {
        if let Some(tile) = self.get_data(x, y) {
            tile.is_diggable()
        } else {
            false
        }
    }

    /// Terrain creation
    /// Needs cleaning (move buildings creation somewhere else)
    pub fn new() -> Terrain {
        let tiles: Vec<Vec<TileType>> = vec![vec![TileType::AIR; WIDTH as usize]; HEIGHT as usize];
        let mut buildings = Vec::new();

        buildings.push(Building::new(RaceType::ANT, BuildingType::Hearth));
        buildings.push(Building::new(
            RaceType::ANT,
            BuildingType::Stockpile(MineralType::IRON),
        ));
        buildings.push(Building::new(
            RaceType::ANT,
            BuildingType::Stockpile(MineralType::ROCK),
        ));
        buildings.push(Building::new(
            RaceType::ANT,
            BuildingType::Stockpile(MineralType::GOLD),
        ));

        buildings.push(Building::new(RaceType::HUMAN, BuildingType::Hearth));
        buildings.push(Building::new(
            RaceType::HUMAN,
            BuildingType::Stockpile(MineralType::IRON),
        ));
        buildings.push(Building::new(
            RaceType::HUMAN,
            BuildingType::Stockpile(MineralType::ROCK),
        ));

        buildings.push(Building::new(
            RaceType::HUMAN,
            BuildingType::Stockpile(MineralType::GOLD),
        ));

        buildings.push(Building::new(RaceType::ALIEN, BuildingType::Hearth));
        buildings.push(Building::new(
            RaceType::ALIEN,
            BuildingType::Stockpile(MineralType::IRON),
        ));
        buildings.push(Building::new(
            RaceType::ALIEN,
            BuildingType::Stockpile(MineralType::ROCK),
        ));

        buildings.push(Building::new(
            RaceType::ALIEN,
            BuildingType::Stockpile(MineralType::GOLD),
        ));

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
                    color: 0x140c07ff,
                    automaton: Automaton {
                        can_replace: vec![
                            //TileType::Mineral(MineralType::ROCK),
                            // TileType::Mineral(MineralType::DIRT),
                            TileType::AIR,
                            TileType::WATER,
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
                    color: 0xa89172ff,
                    automaton: Automaton {
                        can_replace: vec![
                            TileType::Mineral(MineralType::ROCK),
                            TileType::Mineral(MineralType::DIRT), //NO
                                                                  //TileType::AIR,
                        ],
                        birth_limit: 2,
                        death_limit: 5,
                        iterations: 7,
                        perlin_scale: 0.09,
                        perlin_threshold: 0.01,
                        occurence: 0.8,
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
                Mineral {
                    r#type: TileType::WATER,
                    color: 0x334499ff,
                    automaton: Automaton {
                        can_replace: vec![
                            TileType::AIR,
                            TileType::Mineral(MineralType::DIRT),
                            //    TileType::Mineral(MineralType::ROCK),
                            //    TileType::Mineral(MineralType::IRON),
                            //   TileType::Mineral(MineralType::GOLD),
                        ],
                        birth_limit: 1,
                        death_limit: 5,
                        iterations: 10,
                        perlin_scale: 0.06,
                        perlin_threshold: 0.1,
                        occurence: -1.0,
                        max_air_exposure: 5,
                    },
                },
            ],
            buildings,
        }
    }

    /// Use mineral.automaton to modify Perlin noise generated map
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
                        Some(_) => {
                            if noise_value
                                < mineral.automaton.perlin_threshold * mineral.automaton.occurence
                            {
                                self.data[x][y] = mineral.r#type;
                            }
                        }
                        None => panic!("oupsi"),
                    }
                }
            }
        }
        // Application des règles de l'automate cellulaire
        mineral.automaton.apply_rules(self, mineral.r#type);
        //  mineral.automaton.apply_rules(self, TileType::AIR);
    }

    fn clear_tiles(&mut self) {
        self.data = vec![vec![TileType::AIR; WIDTH as usize]; HEIGHT as usize];
    }

    /// Main terrain generation
    /// 
    /// - Fill with AIR
    /// - Add minerals
    /// - Add Buildings
    pub fn generate(&mut self) {
        let minerals_copy: Vec<Mineral> = self.minerals.clone();

        self.clear_tiles();
        for m in minerals_copy {
            self.generate_caves(&m);
        }
        for b in self.buildings.clone() {
            if b.building_type == BuildingType::Hearth {
                // Dig around Hearth (Starting base)
                self.dig_radius(&b.coords, buildings::HOME_STARTING_SIZE);
            }
            self.data[b.coords.x as usize][b.coords.y as usize] =
                TileType::Building(b.building_type);
        }
    }

    /// Dig a circle of radius at center
    /// Replaces with TileType::AIR
    pub fn dig_radius(&mut self, center: &Coords, radius: u32) {
        let (cx, cy) = (center.x as i32, center.y as i32);
        let radius_squared = (radius * radius) as i32;

        for y in (cy - radius as i32)..=(cy + radius as i32) {
            for x in (cx - radius as i32)..=(cx + radius as i32) {
                let dx = x - cx;
                let dy = y - cy;
                // if !(x == center.x && y == center.y) {
                if dx * dx + dy * dy <= radius_squared {
                    if let Some(_) = self.get_data(x as usize, y as usize) {
                        self.data[x as usize][y as usize] = TileType::AIR; // Dig
                    }
                }
                //}
            }
        }
    }
}

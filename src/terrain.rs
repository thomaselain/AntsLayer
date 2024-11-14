extern crate noise;
extern crate sdl2;

use crate::{
    automaton::Automaton,
    buildings::{self, Building, BuildingType},
    coords::Coords,
    minerals::{Mineral, MineralType},
    units::{Item, RaceType},
};

use noise::{NoiseFn, Perlin};
use rand::{self, Rng};

pub const HEIGHT: usize = 300;
pub const WIDTH: usize = HEIGHT;
pub const AIR: Tile = Tile(Some(TerrainType::AIR), None, None);
pub const WATER: Tile = Tile(Some(TerrainType::WATER), None, None);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Mineral(MineralType),
    Building(BuildingType),
    TerrainType(TerrainType),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TerrainType {
    AIR,
    WATER,
}

impl TerrainType {
    pub fn color(self) -> u32 {
        match self {
            TerrainType::AIR => 0x040201ff,
            TerrainType::WATER => 0x334499ff,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tile(
    pub Option<TerrainType>,
    pub Option<Mineral>,
    pub Option<Building>,
);

impl Tile {
    //                 .
    //DANGEROUS ZONE  /|\
    // DONT USE :)   /_o_\
    //
    pub fn buildings(self) -> Building {
        self.2.unwrap()
    }
    pub fn to_tile_type(&mut self) -> (TileType, TileType, TileType) {
        let mut new_tile = (
            TileType::TerrainType(TerrainType::AIR),
            TileType::Mineral(MineralType::MOSS),
            TileType::Building(BuildingType::Hearth),
        );

        match self {
            Tile(Some(terrain), _, _) => new_tile.0 = TileType::TerrainType(*terrain),
            Tile(_, Some(mineral), _) => new_tile.1 = TileType::Mineral(mineral.0),
            Tile(_, _, Some(building)) => new_tile.2 = TileType::Building(building.0),
            _ => {
                todo!("Empty tiles conversion")
            }
        }
        new_tile
    }
    pub fn get_terrain(self) -> Option<TerrainType> {
        self.0
    }
    pub fn get_minerals(self) -> Option<Mineral> {
        self.1
    }
    pub fn get_buildings(self) -> Option<Building> {
        self.2
    }
    pub fn add_single(&mut self, add: TileType) {
        match add {
            TileType::TerrainType(terrain_type) => {
                self.0 = Some(terrain_type);
            }
            TileType::Mineral(mineral_type) => {
                self.1 = Some(Mineral(mineral_type));
            }
            TileType::Building(building_type) => match self.2 {
                Some(building) => {
                    self.2 = Some(Building(building_type, building.1));
                }
                None => todo!("What is this building ???"),
            },
        }
    }
    pub fn add(mut self, add: Tile) {
        match self {
            Tile(None, _, _) => self.0 = add.0,
            Tile(_, None, _) => self.1 = add.1,
            Tile(_, _, None) => self.2 = add.2,
            _ => {}
        };
    }

    /// Counts tiles around (x,y) that have the same type as tile_type
    pub fn count_neighbors(&mut self, terrain: Terrain, tile: Tile, x: usize, y: usize) -> usize {
        let mut count = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                match terrain.get_tile(nx as usize, ny as usize) {
                    Some(Tile(_, Some(mineral), _)) => {
                        if mineral == Mineral(tile.1.unwrap().0) {
                            count += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        count
    }
}

impl TileType {
    pub fn to_ascii(self) -> String {
        match self {
            TileType::Mineral(mineral_type) => mineral_type.to_ascii(),
            _ => " ".to_string(),
        }
    }
    /// Can be harvested
    /// ROCK - for
    /// IRON - for
    /// GOLD - for
    /// MOSS - for breeding
    pub fn is_collectable(self) -> bool {
        match self {
            TileType::Mineral(MineralType::ROCK) => false,
            TileType::Mineral(MineralType::IRON) => false,
            TileType::Mineral(MineralType::GOLD) => false,
            TileType::Mineral(MineralType::MOSS) => true,
            _ => false,
        }
    }
    pub fn into_mineral(self) -> Option<Mineral>{
match self {
    TileType::Mineral(mineral_type) => Some(Mineral(mineral_type)),
    _ => None,
}    }
    pub fn into_item(self) -> Option<Item> {
        if self.is_collectable() {
            Some(Item::Mineral(self))
        } else {
            return None;
        }
    }
}

#[derive(Clone)]
/// Main structure for terrain manipulation and data storage
/// Each mineral in minerals have an automaton with set rules for generation in generate_caves()
/// data contains all map data
pub struct Terrain {
    //pub buildings: Vec<Building>,
    //pub minerals: Vec<Mineral>,
    pub data: Vec<Vec<Tile>>,
    pub automatons: Vec<Automaton>,
}

impl Terrain {
    /// Terrain creation
    /// Needs cleaning (move buildings creation somewhere else)
    pub fn new() -> Terrain {
        let tiles: Vec<Vec<Tile>> = vec![vec![AIR; WIDTH as usize]; HEIGHT as usize];
        let mut buildings = Vec::new();
        let automatons = Vec::new();

        // ANT //
        buildings.push(Building(BuildingType::Hearth, RaceType::ANT));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::IRON),
            RaceType::ANT,
        ));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::ROCK),
            RaceType::ANT,
        ));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::GOLD),
            RaceType::ANT,
        ));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::MOSS),
            RaceType::ANT,
        ));

        // HUMAN //
        buildings.push(Building(BuildingType::Hearth, RaceType::HUMAN));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::IRON),
            RaceType::HUMAN,
        ));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::ROCK),
            RaceType::HUMAN,
        ));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::GOLD),
            RaceType::HUMAN,
        ));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::MOSS),
            RaceType::HUMAN,
        ));

        // ALIEN //
        buildings.push(Building(BuildingType::Hearth, RaceType::ALIEN));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::IRON),
            RaceType::ALIEN,
        ));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::ROCK),
            RaceType::ALIEN,
        ));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::GOLD),
            RaceType::ALIEN,
        ));
        buildings.push(Building(
            BuildingType::Stockpile(MineralType::MOSS),
            RaceType::ALIEN,
        ));

        Terrain {
            data: tiles,
            automatons,
        }
    }

    /// Use mineral.automaton to modify Perlin noise generated map
    pub fn generate_caves(&mut self, automaton: Automaton) {
        let mut rng = rand::thread_rng();
        let noise: Perlin = Perlin::new(rng.gen());
        println!("Color = {:#X}", automaton.mineral_type.color()); // Ajout pour le debug
        if automaton.mineral_type.occurence() == 0.0 {
            return;
        } // Set to 0.0 to skip (for testing)

        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                let tile = if let Some(t) = self.get_tile(x, y) {
                    t
                } else {
                    continue;
                };

                let noise_value = noise.get([
                    x as f64 * automaton.mineral_type.perlin_scale(),
                    y as f64 * automaton.mineral_type.perlin_scale(),
                ]);

                if noise_value
                    < automaton.mineral_type.perlin_threshold() * automaton.mineral_type.occurence()
                {
                    for c_r in &automaton.mineral_type.can_replace() {
                        self.data[x][y] = Tile(
                            self.data[x][y].0,
                            c_r.into_mineral(),
                            self.data[x][y].2,
                        );
                        continue;
                    }
                }

                //match tile {
                //    Tile(_, _, _) => {
                //        for c_r in &automaton.mineral_type.can_replace() {
                //            println!("salut ! {:?} ", automaton.mineral_type.can_replace());
                //            println!("????????");
                //            println!("oh");
                //    }
                //    _ => {}
                //}
            }
        }
        automaton.apply_rules(self);
    }

    fn clear_tiles(&mut self) {
        self.data = vec![vec![AIR; WIDTH as usize]; HEIGHT as usize];
    }

    /// Main terrain generation
    ///
    /// - Fill with AIR
    /// - Add minerals
    /// - Add Buildings
    pub fn generate(&mut self) {
        // Fill with AIR
        self.clear_tiles();

        // Generate terrain layer by layer
        // self.generate_caves(Automaton::new(MineralType::IRON));
        self.generate_caves(Automaton::new(MineralType::ROCK));
        self.generate_caves(Automaton::new(MineralType::GOLD));
        self.generate_caves(Automaton::new(MineralType::DIRT));
        self.generate_caves(Automaton::new(MineralType::MOSS));

        // // Add buildings
        // for b in self.buildings.clone() {
        //     if b.building_type == TileType::Building(BuildingType::Hearth) {
        //         // Dig around Hearth (Starting base)
        //         self.dig_radius(&b.coords, buildings::HOME_STARTING_SIZE);
        //     }
        //     self.data[b.coords.x as usize][b.coords.y as usize].add(Tile(None, None, Some(b)));
        // }
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
                    if let Some(_) = self.get_tile(x as usize, y as usize) {
                        self.data[x as usize][y as usize] = AIR; // Dig
                    }
                }
                //}
            }
        }
    }
}

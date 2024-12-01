mod automaton;
pub(crate) mod map_creation;
pub(crate) mod tile;

use std::fs;

use automaton::Automaton;
use coords::Coords;
use noise::{ NoiseFn, Perlin };
use rand::Rng;
use tile::{
    buildings::{ Buildable, Building },
    minerals::{ gen_params::SETTINGS_PATH, MineralType },
    terrain::TerrainType,
    Tile,
};

use super::units::RaceType;

pub const AIR: Tile = Tile(Some(TerrainType::AIR), None, None);
pub const WATER: Tile = Tile(Some(TerrainType::WATER), None, None);

pub const HEIGHT: usize = 100;
pub const WIDTH: usize = HEIGHT;

#[derive(Clone)]
/// Main structure for terrain manipulation and data storage
/// Each mineral in minerals have an automaton with set rules for generation in generate_caves()
/// data contains all map data
pub struct Map {
    pub data: Vec<Vec<Tile>>,
    gen_params: Vec<Automaton>,
}
impl Map {
    /// Terrain creation
    /// Needs cleaning (move buildings creation somewhere else)
    pub fn new() -> Map {
        let tiles: Vec<Vec<Tile>> = vec![vec![AIR; WIDTH as usize]; HEIGHT as usize];
        let data = fs::read_to_string(SETTINGS_PATH);
        let content = json::parse(&data.expect("!!!")).unwrap();
        let gen_params: Vec<Automaton> = vec![
            Automaton::new(MineralType::ROCK, content.clone()),
            Automaton::new(MineralType::DIRT, content.clone()),
            Automaton::new(MineralType::MOSS, content.clone())
        ];

        Map {
            data: tiles,
            gen_params: gen_params,
        }
    }

    /// Use mineral.automaton to modify Perlin noise generated map
    pub fn generate_caves(&mut self, automaton: &Automaton) {
        let mut rng = rand::thread_rng();
        let noise: Perlin = Perlin::new(rng.gen());
        if !automaton.enable {
            return;
        }
        println!("Generating {}...", automaton.mineral_type.to_str());
        println!("With params : {:?} ", automaton);

        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                for c_r in &automaton.can_replace {
                    match self.get_tile(x, y) {
                        Ok((tile, _)) => {
                            if tile.has(*c_r).is_ok() {
                                let noise_value = noise.get([
                                    (x as f64) * automaton.perlin_scale,
                                    (y as f64) * automaton.perlin_scale,
                                ]);

                                if noise_value < automaton.perlin_threshold {
                                    self.data[x][y] = automaton.mineral_type
                                        .to_tile_type()
                                        .as_tile();
                                }
                                break;
                            }
                        }
                        Err(coords) => panic!("Caves generation failed at {:?}", coords),
                    }
                }
            }
        }
        automaton.apply_rules(self);
    }

    fn clear_tiles(&mut self) {
        self.data = vec![vec![AIR; WIDTH as usize]; HEIGHT as usize];
    }

    pub fn generate(&mut self) -> Result<(), Coords> {
        // Fill with AIR
        self.clear_tiles();

        // Generate terrain layer by layer
        for automaton in self.gen_params.clone() {
            self.generate_caves(&automaton);
        }
        // self.generate_caves(Automaton::new(MineralType::DIRT, content.clone()));
        // self.generate_caves(Automaton::new(MineralType::IRON, content.clone()));
        // self.generate_caves(Automaton::new(MineralType::MOSS, content.clone()));

        self.build_starting_zone(RaceType::ANT).expect("Could not place ANT Starting zone");
        self.build_starting_zone(RaceType::HUMAN).expect("Could not place HUMAN Starting zone");
        self.build_starting_zone(RaceType::ALIEN).expect("Could not place ALIEN Starting zone");

        Ok(())
    }

    fn build(&mut self, building: Building<Buildable<RaceType>>) -> Result<(Tile, Coords), Coords> {
        let mut curr_tile = self
            .get_tile_from_coords(building.coords)
            .ok()
            .expect("Invalid building coords");

        self.set_tile(building.coords, Some(curr_tile.0.set_single(building.to_tile_type())))?;

        self.get_tile_from_coords(building.coords)
    }
    pub fn dig_cell(&mut self, coords: Coords) -> Result<Tile, Coords> {
        if let Ok((tile, _)) = self.get_tile_from_coords(coords) {
            self.data[coords.x() as usize][coords.y() as usize] = AIR; // Dig
            Ok(tile)
        } else {
            Err(coords)
        }
    }
    /// Dig a circle of radius at center
    /// Replaces with TileType::AIR
    pub fn dig_radius(&mut self, center: &Coords, radius: u32) -> Result<(), Coords> {
        let (cx, cy) = (center.x() as i32, center.y() as i32);
        let radius_squared = (radius * radius) as i32;

        for y in cy - (radius as i32)..=cy + (radius as i32) {
            for x in cx - (radius as i32)..=cx + (radius as i32) {
                let dx = x - cx;
                let dy = y - cy;
                // if !(x == center.x && y == center.y) {
                if dx * dx + dy * dy <= radius_squared {
                    if let Ok(tile) = self.get_tile(x as usize, y as usize) {
                        self.data[x as usize][y as usize] = AIR; // Dig
                    } else if !self.check_data(x as usize, y as usize){
                       // return Err(Coords(x, y));
                    }
                }
                //}
            }
        }
        Ok(())
    }
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
    /// Set data[x][y] to tile
    pub fn set_tile(&mut self, coords: Coords, tile: Option<Tile>) -> Result<Tile, Coords> {
        if tile.is_some() && self.clone().get_tile_from_coords(coords).is_ok() {
            self.data[coords.x() as usize][coords.y() as usize] = tile.unwrap();
            Ok(tile.unwrap())
        } else {
            Err(coords)
        }
    }

    /// True if terrain.get_data(x, y) is walkable (see TileType.is_walkable)
    pub fn is_walkable(&self, coords: Coords) -> bool {
        if let Ok((tile, _)) = self.get_tile_from_coords(coords) {
            tile.is_walkable()
        } else {
            false
        }
    }
    pub fn get_tile_from_coords(&self, coords: Coords) -> Result<(Tile, Coords), Coords> {
        self.get_tile(coords.x() as usize, coords.y() as usize)
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Result<(Tile, Coords), Coords> {
        if self.check_data(x, y) {
            Ok((self.data[x][y], Coords(x as i32, y as i32)))
        } else {
            Err(Coords(x as i32, y as i32))
        }
    }
}

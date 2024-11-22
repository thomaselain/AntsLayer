mod automaton;
pub (crate) mod tile;
pub (crate) mod map_creation;

use automaton::Automaton;
use coords::Coords;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use tile::{buildings::{Buildable, Building}, minerals::MineralType, terrain::TerrainType, Tile};

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
}
impl Map {
    /// Terrain creation
    /// Needs cleaning (move buildings creation somewhere else)
    pub fn new() -> Map {
        let tiles: Vec<Vec<Tile>> = vec![vec![AIR; WIDTH as usize]; HEIGHT as usize];
        Map { data: tiles }
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
                let tile = self.get_tile(x, y);
                let mut can_replace: bool = false;

                for c_r in &automaton.can_replace {
                    if tile == Ok((c_r.as_tile(), Coords(x as i32, y as i32))) {
                        can_replace = true;
                    }
                }
                let noise_value = noise.get([
                    x as f64 * automaton.perlin_scale,
                    y as f64 * automaton.perlin_scale,
                ]);
                if can_replace {
                    match tile {
                        Ok(_) => {
                            if noise_value < automaton.perlin_threshold * automaton.occurence {
                                self.data[x][y] = automaton.mineral_type.to_tile_type().as_tile();
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
        self.generate_caves(Automaton::new(MineralType::IRON));
        self.generate_caves(Automaton::new(MineralType::ROCK));
        self.generate_caves(Automaton::new(MineralType::GOLD));
        self.generate_caves(Automaton::new(MineralType::DIRT));
        self.generate_caves(Automaton::new(MineralType::MOSS));

        self.build_starting_zone(RaceType::ANT)
            .expect("Could not place ANT Starting zone");
        self.build_starting_zone(RaceType::HUMAN)
            .expect("Could not place HUMAN Starting zone");
        self.build_starting_zone(RaceType::ALIEN)
            .expect("Could not place ALIEN Starting zone");

        Ok(())
    }

    fn build(&mut self, building: Building<Buildable<RaceType>>) -> Result<(Tile, Coords), Coords> {
        let mut curr_tile = self
            .get_tile_from_coords(building.coords)
            .ok()
            .expect("Invalid building coords");

        self.set_tile(
            building.coords,
            Some(curr_tile.0.add_single(building.to_tile_type())),
        )?;

        self.get_tile_from_coords(building.coords)
    }
    /// Dig a circle of radius at center
    /// Replaces with TileType::AIR
    pub fn dig_radius(&mut self, center: &Coords, radius: u32) -> Result<(), Coords> {
        let (cx, cy) = (center.x() as i32, center.y() as i32);
        let radius_squared = (radius * radius) as i32;

        for y in (cy - radius as i32)..=(cy + radius as i32) {
            for x in (cx - radius as i32)..=(cx + radius as i32) {
                let dx = x - cx;
                let dy = y - cy;
                // if !(x == center.x && y == center.y) {
                if dx * dx + dy * dy <= radius_squared {
                    if let Ok(tile) = self.get_tile(x as usize, y as usize) {
                        self.data[x as usize][y as usize] = AIR; // Dig
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

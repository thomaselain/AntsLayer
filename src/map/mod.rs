mod automaton;
pub(crate) mod buildings;
pub(crate) mod minerals;
pub(crate) mod terrain;

mod map_creation;

use automaton::Automaton;
use buildings::{Buildable, Building, BuildingType, Content, Stockpile};
use coords::Coords;
use minerals::{Mineral, MineralType};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use terrain::TerrainType;

use crate::units::{Item, RaceType};

pub const AIR: Tile = Tile(Some(TerrainType::AIR), None, None);
pub const WATER: Tile = Tile(Some(TerrainType::WATER), None, None);

pub const HEIGHT: usize = 150;
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
                    if tile == Ok(c_r.as_tile()) {
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
        // self.generate_caves(Automaton::new(MineralType::IRON));
        self.generate_caves(Automaton::new(MineralType::ROCK));
        self.generate_caves(Automaton::new(MineralType::GOLD));
        self.generate_caves(Automaton::new(MineralType::DIRT));
        //self.generate_caves(Automaton::new(MineralType::MOSS));

        self.build_starting_zone(RaceType::ANT)
            .expect("Could not place ANT Starting zone");
        self.build_starting_zone(RaceType::HUMAN)
            .expect("Could not place HUMAN Starting zone");
        self.build_starting_zone(RaceType::ALIEN)
            .expect("Could not place ALIEN Starting zone");

    
        Ok(())
    }


    fn build(
        &mut self,
        building: Building<Buildable<RaceType>>,
    ) -> Result<Tile, Coords> {
        let mut curr_tile = self
            .get_tile_from_coords(building.coords)
            .ok()
            .expect("Invalid building coords");

        self.set_tile(building.coords, Some(curr_tile.add_single(building.to_tile_type())))?;

        self.get_tile_from_coords(building.coords   )
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
        if let Ok(tile) = self.get_tile_from_coords(coords) {
            tile.is_walkable()
        } else {
            false
        }
    }
    pub fn get_tile_from_coords(&self, coords: Coords) -> Result<Tile, Coords> {
        self.get_tile(coords.x() as usize, coords.y() as usize)
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Result<Tile, Coords> {
        if self.check_data(x, y) {
            Ok(self.data[x][y])
        } else {
            Err(Coords(x as i32, y as i32))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Mineral(MineralType),
    Building(Buildable<RaceType>),
    TerrainType(TerrainType),
}

impl TileType {
    pub fn is_walkable(self) -> bool {
        match self {
            TileType::TerrainType(TerrainType::AIR) => true,
            TileType::TerrainType(TerrainType::WATER) => true,
            TileType::Mineral(MineralType::MOSS) => true,
            TileType::Building(_) => true,
            _ => false,
        }
    }
    pub fn get_mineral_type(self) -> Result<MineralType, Self> {
        match self {
            TileType::Mineral(mineral_type) => Ok(mineral_type),
            _ => Err(self),
        }
    }
    pub fn get_building_type(self) -> Option<BuildingType> {
        match self {
            TileType::Building(buildable) => buildable.to_tile_type().get_building_type(),
            _ => None,
        }
    }
    pub fn get_terrain_type(self) -> Option<TerrainType> {
        match self {
            TileType::TerrainType(terrain_type) => Some(terrain_type),
            _ => None,
        }
    }

    pub fn as_tile(self) -> Tile {
        let mut tile = Tile::new();
        match self {
            TileType::TerrainType(terrain_type) => {
                tile.add_single(terrain_type.to_tile_type());
            }
            TileType::Mineral(mineral_type) => {
                tile.add_single(mineral_type.to_tile_type());
            }
            TileType::Building(buildable) => {
                tile.add_single(TileType::Building { 0: buildable });
            }
        };
        tile
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tile(
    pub Option<TerrainType>,
    pub Option<Mineral>,
    pub Option<Buildable<RaceType>>,
);

impl Tile {
    pub fn new() -> Tile {
        Tile(None, None, None)
    }
    pub fn eq(self, other: Option<Tile>) -> bool {
        if other.is_some()
            && self.0 == other.unwrap().0
            && self.1 == other.unwrap().1
            && self.2 == other.unwrap().2
        {
            true;
        }
        false
    }
    pub fn has(self, other: TileType) -> bool {
        // Terrain comparaison
        if self.0.is_some() && self.0.unwrap().to_tile_type() == other {
            true;
        }
        // Mineral comparaison
        if self.1.is_some() && self.1.unwrap().0.to_tile_type() == other {
            true;
        }
        // Building comparaison
        if self.2.is_some() && other.get_building_type().is_some() {
            if self.2.unwrap().to_tile_type() == other {
                true;
            } else if self.get_buildable().unwrap().building_type()
                == other.get_building_type().unwrap()
            {
                true;
            }
        }
        false
    }

    pub fn is_some(self) -> bool {
        if self.0.is_some() && self.1.is_some() && self.2.is_some() {
            true;
        }
        false
    }
    pub fn to_tile_type(&mut self) -> (Option<TileType>, Option<TileType>, Option<TileType>) {
        let mut new_tile = (None, None, None);

        match (self.0, self.1, self.2) {
            (Some(terrain), _, _) => new_tile.0 = Some(TileType::TerrainType(terrain)),
            (_, Some(mineral), _) => new_tile.1 = Some(TileType::Mineral(mineral.0)),
            (_, _, Some(building)) => todo!("Building color"), //new_tile.2 = Some(building.to_tile_type()),
            _ => {
                todo!("Empty tiles conversion")
            }
        }
        new_tile
    }
    pub fn get_terrain(self) -> Result<TerrainType, Tile> {
        match self.0 {
            Some(terrain) => Ok(terrain),
            _ => Err(self),
        }
    }
    pub fn get_mineral(self) -> Result<Mineral, Tile> {
        match self.1 {
            Some(mineral) => Ok(mineral),
            _ => Err(self),
        }
    }
    pub fn get_mineral_type(self) ->Result<MineralType, Tile>{
        match self.get_mineral() {
            a_mineral => Ok(a_mineral.ok().expect("Should be a mineral").0),
            _ => Err(self),
        }
    }
    pub fn get_buildable(self) -> Result<Buildable<RaceType>, Tile> {
        match self.2 {
            Some(buildable) => Ok(buildable),
            _ => Err(self),
        }
    }

    pub fn add_single(&mut self, tile_type: TileType) -> Tile {
        match tile_type {
            TileType::TerrainType(terrain_type) => {
                self.0 = Some(terrain_type);
            }
            TileType::Mineral(mineral_type) => {
                self.1 = Some(Mineral(mineral_type));
            }
            TileType::Building(buildable) => {
                self.2 = Some(buildable);
            }
        }
        *self
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
    pub fn count_neighbors(&mut self, terrain: Map, tile: Tile, x: usize, y: usize) -> usize {
        let mut count = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                match terrain.get_tile(nx as usize, ny as usize) {
                    Ok(Tile(_, Some(mineral), _)) => {
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
    pub fn into_mineral(self) -> Option<Mineral> {
        match self {
            TileType::Mineral(mineral_type) => Some(Mineral(mineral_type)),
            _ => None,
        }
    }
    pub fn into_item(self) -> Option<Item> {
        if self.is_collectable() {
            Some(Item::Mineral(self))
        } else {
            return None;
        }
    }
}

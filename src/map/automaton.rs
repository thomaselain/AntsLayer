use super::{minerals::MineralType, terrain::TerrainType, Map, Tile, TileType, HEIGHT, WIDTH};

extern crate automata;

// Structure Automaton associée à chaque mineral
#[derive(Clone, PartialEq, Debug)]
pub struct Automaton {
    /// Automatons associated MineralType
    pub mineral_type: MineralType,
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
    pub fn new(mineral_type: MineralType) -> Automaton {
        Automaton {
            mineral_type,
            can_replace: mineral_type.can_replace(),
            max_air_exposure: mineral_type.max_air_exposure(),
            birth_limit: mineral_type.birth_limit(),
            death_limit: mineral_type.death_limit(),
            iterations: mineral_type.iterations(),
            perlin_scale: mineral_type.perlin_scale(),
            perlin_threshold: mineral_type.perlin_threshold(),
            occurence: mineral_type.occurence(),
        }
    }

    /// Automaton rules
    pub fn apply_rules(&self, map: &mut Map) {
        for _ in 0..self.iterations {
            let mut new_data = map.data.clone();
            for x in 1..(WIDTH as usize - 1) {
                for y in 1..(HEIGHT as usize - 1) {
                    let mut can_replace: bool = false;
                    let mut current_tile: Tile;
                    if let Ok(curr) = map.get_tile(x, y) {
                        current_tile = curr;
                    } else {
                        continue;
                    }
                    for c_r in &self.can_replace {
                        if map.get_tile(x, y).unwrap().to_tile_type().1 == Some(*c_r) {
                            can_replace = true;
                        }
                    }

                    let count_same = current_tile.count_neighbors(map.clone(), current_tile, x, y);
                    let count_air = current_tile.count_neighbors(
                        map.clone(),
                        Tile {
                            0: Some(TerrainType::AIR),
                            1: current_tile.1,
                            2: current_tile.2,
                        },
                        x,
                        y,
                    );

                    if can_replace {
                        if count_same < self.death_limit {
                            new_data[x][y] = Tile {
                                0: Some(TerrainType::AIR),
                                1: current_tile.1,
                                2: current_tile.2,
                            };
                        }
                    } else if current_tile.0 == Some(TerrainType::AIR)
                        && count_air <= self.max_air_exposure
                    {
                        // loop self.priority_list to find out if we replace
                        if count_same > self.birth_limit {
                            new_data[x][y] = Tile {
                                0: None,
                                1: current_tile.1,
                                2: current_tile.2,
                            };
                        }
                    }
                }
            }
            map.data = new_data;
        }
    }
}

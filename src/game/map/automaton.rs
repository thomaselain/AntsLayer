use super::{
    tile::{minerals::MineralType, Tile, TileType},
    Map, TerrainType, AIR, HEIGHT, WIDTH,
};
use json::JsonValue;

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
    pub enable: bool,
}

impl Automaton {
    pub fn new(mineral_type: MineralType, data: JsonValue) -> Automaton {
        let settings = &data[mineral_type.to_str()];
        let mut can_replace: Vec<TileType> = vec![];

        for can in settings["can replace"].members() {
            can_replace.push(
                TileType::from_str(can.as_str().unwrap())
                    .expect("Invalid TileType in  automaton settings"),
            );
        }

        Automaton {
            mineral_type,
            can_replace: can_replace.clone().to_vec(),
            enable: settings["enable"].as_bool().unwrap(),

            perlin_scale: settings["perlin"]["scale"].as_f64().unwrap(),
            perlin_threshold: settings["perlin"]["threshold"].as_f64().unwrap(),

            birth_limit: settings["automaton"]["birth limit"].as_usize().unwrap(),
            death_limit: settings["automaton"]["death limit"].as_usize().unwrap(),
            max_air_exposure: settings["automaton"]["max air exposure"]
                .as_usize()
                .unwrap(),
            iterations: settings["automaton"]["iterations"].as_usize().unwrap(),
        }
    }

    /// Automaton rules
    pub fn apply_rules(&self, map: &mut Map) {
        for _ in 0..self.iterations {
            let mut new_data = map.data.clone();
            for x in 1..WIDTH {
                for y in 1..HEIGHT {
                    match map.get_tile(x, y) {
                        Ok((mut tile, _)) => {
                            let count_same = tile.count_neighbors(map.clone(), tile, x, y);
                            let count_air = tile.count_neighbors(
                                map.clone(),
                                Tile(Some(TerrainType::AIR), tile.1, tile.2),
                                x,
                                y,
                            );
                            if count_same < self.death_limit {
                                new_data[x][y].set_single(TileType::TerrainType(TerrainType::AIR));
                            } else if tile == AIR && count_air <= self.max_air_exposure {
                                if count_same > self.birth_limit {
                                    new_data[x][y]
                                        .set_single(TileType::TerrainType(TerrainType::AIR));
                                }
                            }
                        }
                        Err(coords) => panic!("Caves generation failed at {:?}", coords),
                    }
                }
            }
            map.data = new_data;
        }
    }
}

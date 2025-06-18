use std::{ collections::HashMap, sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex } };

use noise::Fbm;

use crate::{ ant::AntManager, chunk::{ biomes::{ NoiseParams } }, renderer::Renderer };

use super::{
    generation::{ MapShape, STARTING_AREA, STARTING_MAP_SHAPE },
    tile::Tile,
    Chunk,
    WIDTH,
};
pub type WorldNoise = Arc<[NoiseParams; 10]>;
pub struct Manager {
    pub world_noise: WorldNoise,
    pub rx: Sender<LoadedChunk>,
    pub tx: Receiver<LoadedChunk>,
    pub pending_chunks: Vec<LoadedChunk>,
    pub loaded_chunks: HashMap<(i32, i32), LoadedChunk>,
}

#[derive(Clone)]
pub struct LoadedChunk {
    pub pos: (i32, i32),
    pub c: Arc<Mutex<Chunk>>,
}
impl Manager {
    pub fn render(&mut self, renderer: &mut Renderer, a_mngr: &AntManager, timestamp: f64) {
        renderer.filter_visible_chunks(&mut self.loaded_chunks);

        for (pos, loaded) in self.loaded_chunks.clone() {
            if renderer.is_chunk_on_screen(pos) {
                loaded.render(renderer, &a_mngr.ants, timestamp);
            }
        }
    }

    pub fn tile_at(&self, p: (i32, i32, i32)) -> Option<Tile> {
        let chunk_pos = (p.0 / (WIDTH as i32), p.1 / (WIDTH as i32));
        for (_pos, loaded_chunk) in &self.loaded_chunks {
            if loaded_chunk.pos == chunk_pos {
                return Some(loaded_chunk.c.lock().unwrap().get(p));
            }
        }
        // Could not find this tile in loaded chunks
        None
    }
}

impl Manager {
    pub const SURFACE: usize = 0;
    pub const VARIATIONS: usize = 1;
    pub const DETAIL: usize = 2;
    pub const CAVES: usize = 3;
    pub const TUNNELS: usize = 4;
    pub const VEINS: usize = 5;
    pub const HUMIDITY: usize = 6;
    pub const ELEVATION: usize = 7;
    pub const ROUGHNESS: usize = 8;
    pub const TEMPERATURE: usize = 9;

    fn empty() -> Self {
        let (rx, tx) = mpsc::channel();
        Self {
            world_noise: Arc::new([
                // Surface
                NoiseParams {
                    fbm: Fbm::new(1),
                    octaves: 7,
                    frequency: 1.3,
                    lacunarity: 2.0,
                    persistence: 1.5,
                    scale: 0.045,
                },
                // Variations
                NoiseParams {
                    fbm: Fbm::new(1),
                    octaves: 1,
                    frequency: 1.0,
                    lacunarity: 2.0,
                    persistence: 1.0,
                    scale: 1.0, // IS MULTIPLIED BY SURFACE SCALE
                },
                // Details
                NoiseParams {
                    fbm: Fbm::new(1),
                    octaves: 4,
                    frequency: 1.0,
                    lacunarity: 2.0,
                    persistence: 1.0,
                    scale: 1.0, // IS MULTIPLIED BY SURFACE SCALE
                },
                // Caves
                NoiseParams {
                    fbm: Fbm::new(64),
                    octaves: 1,
                    frequency: 1.9,
                    lacunarity: 2.0,
                    persistence: 1.5,
                    scale:0.9, // IS MULTIPLIED BY SURFACE SCALE
                },
                // Tunnels
                NoiseParams {
                    fbm: Fbm::new(65),
                    octaves: 1,
                    frequency: 1.0,
                    lacunarity: 2.0,
                    persistence: 0.1,
                    scale: 1.0, // IS MULTIPLIED BY SURFACE SCALE
                },
                // Layers
                NoiseParams {
                    fbm: Fbm::new(3),
                    octaves: 4,
                    frequency: 0.02,
                    lacunarity: 2.0,
                    persistence: 0.4,
                    scale: 0.09,
                },
                // HUMIDITY
                NoiseParams::default(),
                // ELEVATION
                NoiseParams {
                    fbm: Fbm::new(333),
                    octaves: 3,
                    frequency: 1.1,
                    lacunarity: 2.0,
                    persistence: 1.1,
                    scale: 0.0001,
                },
                // ROUGHNESS
                NoiseParams::default(),
                // TEMPERATURE
                NoiseParams {
                    fbm: Fbm::new(33),
                    octaves: 3,
                    frequency: 1.2,
                    lacunarity: 2.0,
                    persistence: 0.99,
                    scale: 0.001,
                },
            ]),
            rx,
            tx,
            pending_chunks: vec![],
            loaded_chunks: HashMap::new(),
        }
    }
}
impl Default for Manager {
    fn default() -> Self {
        let mut mngr = Manager::empty();

        let handles = match STARTING_MAP_SHAPE {
            MapShape::RECT => {
                mngr.generate_range(
                    -STARTING_AREA..STARTING_AREA,
                    -(STARTING_AREA / 2)..STARTING_AREA / 2
                )
            }
            MapShape::SQUARE => {
                mngr.generate_range(-STARTING_AREA..STARTING_AREA, -STARTING_AREA..STARTING_AREA)
            }
            MapShape::ROUND => { todo!("Round starting map") }
        };

        for h in handles {
            let chunk = h.join().unwrap();
            mngr.loaded_chunks.insert(chunk.pos, chunk);
        }

        return mngr;
    }
}

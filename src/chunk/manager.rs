use std::{ collections::HashMap, sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex } };

use noise::Fbm;

use crate::{
    ant::AntManager,
    chunk::{ biomes::{ Id, NoiseParams, Params }, generation::WorldNoise },
    renderer::Renderer,
};

use super::{
    generation::{ MapShape, STARTING_AREA, STARTING_MAP_SHAPE },
    tile::Tile,
    Chunk,
    CHUNK_WIDTH,
};
pub const AMOUNT_OF_BIOMES: usize = 4;
pub type BiomeNoise = [Params; AMOUNT_OF_BIOMES];

pub struct Manager {
    pub biomes: BiomeNoise,
    pub world_noise: WorldNoise,
    pub rx: Sender<LoadedChunk>,
    pub tx: Receiver<LoadedChunk>,
    pub pending_chunks: Vec<LoadedChunk>,
    pub loaded_chunks: HashMap<(i32, i32), LoadedChunk>,
}

#[derive(Clone)]
pub struct LoadedChunk {
    pub biome_id: crate::chunk::biomes::Id,
    pub pos: (i32, i32),
    pub c: Arc<Mutex<Chunk>>,
}
impl Manager {
    pub fn render(&mut self, renderer: &mut Renderer, a_mngr: &AntManager, timestamp: f64) {
        renderer.filter_visible_chunks(&mut self.loaded_chunks);

        for (_pos, loaded) in self.loaded_chunks.clone() {
            loaded.render(renderer, &a_mngr.ants, timestamp);
        }
    }
    pub fn biome_at(&self, (x, y): (i32, i32)) -> Params {
        let v = self.world_noise[Manager::SURFACE].get((
            //
            x as f64,
            //
            y as f64,
            //
            0.0,
        ));
        let id = ((((AMOUNT_OF_BIOMES as f64) + v) / (AMOUNT_OF_BIOMES as f64)) as i32).into();

        if let Some(biome) = self.biomes.iter().find(|b| { b.id == id }) {
            biome.clone()
        } else {
            Params::default()
        }
    }
    pub fn tile_at(&self, p: (i32, i32, i32)) -> Option<Tile> {
        let chunk_pos = (p.0 / (CHUNK_WIDTH as i32), p.1 / (CHUNK_WIDTH as i32));
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
    pub const LAYERS: usize = 4;

    fn empty() -> Self {
        let (rx, tx) = mpsc::channel();
        Self {
            biomes: Params::all(),
            world_noise: Arc::new([
                // Surface
                NoiseParams {
                    fbm: Fbm::new(1),
                    octaves: 4,
                    frequency: 0.04,
                    lacunarity: 2.0,
                    persistence: 0.5,
                    scale: 0.05,
                },
                // Variations
                NoiseParams {
                    fbm: Fbm::new(1),
                    octaves: 4,
                    frequency: 0.5,
                    lacunarity: 2.0,
                    persistence: 0.5,
                    scale: 0.025,
                },
                // Details
                NoiseParams {
                    fbm: Fbm::new(1),
                    octaves: 4,
                    frequency: 0.5,
                    lacunarity: 2.0,
                    persistence: 0.5,
                    scale: 0.1,
                },
                // Caves
                NoiseParams {
                    fbm: Fbm::new(2),
                    octaves: 4,
                    frequency: 0.5,
                    lacunarity: 2.0,
                    persistence: 0.9,
                    scale: 0.18,
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

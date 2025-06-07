use std::{ collections::HashMap, sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex } };

use crate::{ ant::AntManager, chunk::biomes::{ Id, NoiseParams, Params }, renderer::Renderer };

use super::{
    generation::{ MapShape, STARTING_AREA, STARTING_MAP_SHAPE },
    tile::Tile,
    Chunk,
    CHUNK_WIDTH,
};

pub struct Manager {
    pub biomes: Vec<Params>,
    pub biome_noise: NoiseParams,
    pub rx: Sender<LoadedChunk>,
    pub tx: Receiver<LoadedChunk>,
    pub pending_chunks: Vec<LoadedChunk>,
    pub loaded_chunks: HashMap<(i32,i32),LoadedChunk>,
}

#[derive(Clone)]
pub struct LoadedChunk {
    pub biome_id: crate::chunk::biomes::Id,
    pub pos: (i32, i32),
    pub c: Arc<Mutex<Chunk>>,
}
impl Manager {
    pub fn render(&mut self, renderer: &mut Renderer, a_mngr: &AntManager, timestamp: f64) {
        renderer.filter_visible_chunks(self.loaded_chunks.clone());

        for (_pos, loaded) in self.loaded_chunks.clone() {
            loaded.render(renderer, &a_mngr.ants, timestamp);
        }
    }
    pub fn biome_at(&self, (x, y): (i32, i32)) -> Params {
        let v = self.biome_noise.get((
            //
            x as f64,
            //
            y as f64,
            //
            0.0,
        ));
        let id = match v {
            -1.0..-0.5 => { Id::Ocean }
            -0.5..0.0 => { Id::Coast }
            0.0..0.5 => { Id::Plain }
            0.5..1.0 => { Id::Mountain }
            _ => { Id::Mountain }
        };
        if let Some(biome) = self.biomes.iter().find(|b| { b.id == id }) {
            biome.clone()
        } else {
            Params::default()
        }
    }
    pub fn tile_at(&self, p: (i32, i32, i32)) -> Option<Tile> {
        let chunk_pos = (p.0 / (CHUNK_WIDTH as i32), p.1 / (CHUNK_WIDTH as i32));
        for (_pos,loaded_chunk) in &self.loaded_chunks {
            if loaded_chunk.pos == chunk_pos {
                return Some(loaded_chunk.c.lock().unwrap().get(p));
            }
        }
        // Could not find this tile in loaded chunks
        None
    }
}
impl Manager {
    fn empty() -> Self {
        let (rx, tx) = mpsc::channel();
        Self {
            biomes: Params::all(),
            biome_noise: NoiseParams::biomes(),
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

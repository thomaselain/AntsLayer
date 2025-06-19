use std::{ collections::HashMap, sync::{ mpsc::{ Receiver, Sender }, Arc, Mutex } };

use crate::{ ant::{colony::Colony, AntManager}, chunk::biomes::NoiseParams, renderer::Renderer };

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
            if renderer.is_chunk_on_screen(pos) && a_mngr.colonies.len() > 0 {
                loaded.render(renderer, &a_mngr.colonies, timestamp);
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

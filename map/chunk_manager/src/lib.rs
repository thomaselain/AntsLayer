use std::{ collections::{ HashMap, HashSet }, sync::{ mpsc::{ Receiver, Sender }, Arc, Mutex } };

use chunk::{thread::Status, Chunk};
use coords::aliases::{ ChunkPos, TilePos };

pub mod chunk_manager;
mod tests;

/// #
pub struct ChunkManager {
    pub sndr: Arc<Mutex<Sender<(ChunkPos, Status)>>>,
    pub rcvr: Arc<Mutex<Receiver<(ChunkPos, Status)>>>,
    pub loaded_chunks: HashMap<ChunkPos, Status>,
    pub visible_chunks: HashSet<ChunkPos>,
}

/// Update chunks
pub trait Update<Map, Camera> {
    fn update(&mut self, map: &mut Map, camera: &Camera);
}

/// Draw
pub trait Draw<Renderer, Camera> {
    fn draw(&self, renderer: &mut Renderer, camera: &Camera);
}

/// Draw all map
pub trait DrawAll<Map, Renderer, Camera> {
    fn draw_all(&mut self, map: &mut Map, renderer: &mut Renderer, camera: &Camera);
}

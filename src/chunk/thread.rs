use std::{ sync::{ mpsc::channel, Arc, Mutex }, time::Duration };

use crate::{ chunk::{ tile::Tile }, Game, Job };

use super::{ Chunk, ChunkContent, CHUNK_WIDTH };

#[derive(Clone)]
pub struct LoadedChunk {
    pub pos: (i32, i32),
    pub chunk: Arc<Mutex<Chunk>>,
}
impl LoadedChunk {
    // Checks if this xyz is in this chunk (un peu crado)
    pub fn has(&self, (x, y, _z): (i32, i32, i32)) -> bool {
        let (x_min, x_max) = (
            self.pos.0 * (CHUNK_WIDTH as i32),
            (self.pos.0 + 1) * (CHUNK_WIDTH as i32) - 1,
        );
        let (y_min, y_max) = (
            self.pos.1 * (CHUNK_WIDTH as i32),
            (self.pos.1 + 1) * (CHUNK_WIDTH as i32) - 1,
        );

        if x > x_min && x < x_max && y > y_min && y < y_max {
            true
        } else {
            false
        }
    }
}
impl LoadedChunk {
    pub fn access_chunk(&self) -> Chunk {
        if let Some(c) = self.chunk.lock().ok() { *c } else { panic!("Failed to lock chunk") }
    }
    pub fn access_content(&self) -> ChunkContent {
        if let Some(c) = self.chunk.lock().ok() {
            c.content
        } else {
            panic!("Failed to lock chunk")
        }
    }
    pub fn access_tile(&self, pos: (i32, i32, i32)) -> Tile {
        if let Some(c) = self.chunk.lock().ok() {
            c.content[pos]
        } else {
            panic!("Failed to lock chunk")
        }
    }

    pub fn new(pos: (i32, i32), chunk: Chunk) -> Self {
        LoadedChunk {
            pos,
            chunk: Arc::new(Mutex::new(chunk)),
        }
    }
}

impl<'ttf> Game<'ttf> {
    pub fn request_generate_chunk(&self, pos: (i32, i32)) {
        let (tx, rx) = channel::<ChunkContent>();
        let arc_content = Arc::new(Mutex::new(ChunkContent::new()));
        let arc_clone = arc_content.clone();
    
        self.job_sender.send(Job::GenerateChunk { pos, target: arc_clone }).unwrap();
    
        // Stocker le receiver dans pending_chunks
        self.chunk_manager.lock().unwrap().pending_chunks.insert(pos, rx);
    }
    }

use crate::chunk::manager::LoadedChunk;

use super::Renderer;

// Camera
impl<'ttf> Renderer<'ttf> {
    /// Filtre la liste des LoadedChunk pour ne garder que ceux visibles
    pub fn visible_chunks(&self, chunks: Vec<LoadedChunk>) -> Vec<LoadedChunk> {
        let mut v = vec![];
        for c in chunks {
            if self.is_chunk_on_screen(c.pos) {

                // Should avoid cloning
                v.push(c.clone());
            }
        }
        v
    }

    pub fn zoom_in(&mut self) -> Result<(), ()> {
        self.tile_size += 1;
        Ok(())
    }
    pub fn zoom_out(&mut self) -> Result<(), ()> {
        self.tile_size -= 1;
        Ok(())
    }
}

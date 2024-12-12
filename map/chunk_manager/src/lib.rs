use std::collections::HashMap;

use chunk::threads::Status;

pub mod chunk_manager;

/// # For Game implementation
/// Clear chunks that are not seen by the camera
pub trait Clear<Map, Camera> {
    fn clear_out_of_range(&mut self, visible_chunks: HashMap<(i32, i32), Status>);
}

/// Update chunks
pub trait Update<Map, Camera> {
    fn update(&mut self, map: &mut Map, camera: &Camera);
}

/// Draw
pub trait Draw<Renderer, Camera> {
    fn draw(&self, renderer: &mut Renderer, camera:&Camera);
}

/// #
#[derive(Clone)]
pub struct ChunkManager {
    // pub receiver: Receiver<Chunk>,

    // pub chunks: HashMap<(i32, i32), Chunk>, // Clé : coordonnées du chunk
    pub chunks: HashMap<(i32, i32), Status>, // Modifié pour inclure le statut
}

#[cfg(test)]
mod tests {
    use chunk::threads::Status;

    use crate::ChunkManager;

    #[test]
    fn test_chunk_manager_empty() {
        // Crée un ChunkManager vide
        let chunk_manager = ChunkManager::new();

        // Assure qu'il n'y a pas de chunks chargés au début
        assert!(chunk_manager.chunks.is_empty());
    }
}

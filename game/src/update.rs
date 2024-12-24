use biomes::BiomeConfig;
use chunk::{ thread::Status, Chunk, ChunkPath };
use map::Map;

use crate::Game;

impl Game {
    pub fn receive_chunks(&mut self) {
        let mut mngr = self.chunk_manager.lock().unwrap();

        // Vérifiez les chunks reçus via le receiver
        while let Ok((key, status)) = self.rcvr.recv_timeout(self.tick_rate) {
            match status {
                Status::Ready(ref chunk) | Status::Visible(ref chunk) => {
                    // let mut map = self.map.clone().unwrap();
                    chunk.save(ChunkPath::new(&self.map.clone().unwrap().path, key)).unwrap();
                    mngr.loaded_chunks.insert(key, status.clone());
                    self.map.clone().unwrap().add_chunk(key, chunk.clone()).unwrap();
                }
                Status::Pending => {
                    eprintln!("chunk ({},{}) is still waiting", key.x(), key.y());
                }
                _ => {
                    eprintln!("Statut inconnu pour le chunk {:?}: {:?}", key, status);
                }
            }
        }
    }
    // Mettre à jour les unités, la carte, les ressources, etc.
    pub fn update_game_logic(&mut self) {
        if self.map.is_some() {
            self.receive_chunks();
        }
        // Vérifiez régulièrement si des chunks en `Pending` doivent être relancés
    }

    // Mettre à jour les animations, états visuels, etc.
    pub fn update_visuals(&mut self) {
        if self.map.is_some() {
            let mut mngr = self.chunk_manager.lock().unwrap();
            mngr.visible_chunks = Map::visible_chunks(&self.camera);
            for key in mngr.visible_chunks.clone() {
                let path = ChunkPath::new(&self.map.clone().unwrap().path, key);

                match mngr.load_chunk(path) {
                    Ok((key, chunk)) => {
                        mngr.loaded_chunks.insert(key, Status::Ready(chunk));
                    }
                    Err((key, _e)) => {
                        eprintln!("Cannot load {}, generating new chunk", key);
                        Chunk::generate_async(
                            key,
                            self.map.clone().unwrap().seed,
                            BiomeConfig::default(),
                            self.sndr.clone()
                        );
                    }
                }
            }
        }
    }
}

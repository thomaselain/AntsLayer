use chunk::{ thread::Status, ChunkPath };
use map::Map;

use crate::Game;

impl Game {
    pub fn receive_chunks(&mut self) {
        let mut mngr = self.chunk_manager.lock().unwrap();

        // Vérifiez les chunks reçus via le receiver
        while let Ok((key, status)) = self.rcvr.recv_timeout(self.tick_rate) {
            match status {
                Status::Ready(ref chunk) | Status::Visible(ref chunk) => {
                    let mut map = self.map.clone().unwrap();
                    chunk.save(ChunkPath::new(&map.path, key)).unwrap();
                    mngr.loaded_chunks.insert(key, status.clone());
                    map.add_chunk(key, chunk.clone()).unwrap();
                }
                Status::Pending => {}
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
                // eprintln!("{:?}", key);
                
                match mngr.load_chunk(path) {
                    Ok((key, status)) => {
                        mngr.loaded_chunks.insert(key, status);
                    }
                    Err(_e) => {}
                }
            }
        }
    }
}

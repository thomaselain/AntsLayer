use chunk::{ thread::Status, ChunkPath };
use coords::Coords;
use map::{ renderer::{ screen_coords_tile, tile_screen_coords }, Map };

use crate::Game;

impl Game {
    pub fn receive_chunks(&mut self) {
        let mut mngr = self.chunk_manager.lock().unwrap();

        // Vérifiez les chunks reçus via le receiver
        while let Ok((key, status)) = self.rcvr.recv_timeout(self.tick_rate) {
            match status {
                Status::Ready(ref chunk) | Status::Visible(ref chunk) => {
                    let mut map = self.map.clone().unwrap();
                    chunk.save(ChunkPath::build(&map.path, key).unwrap()).unwrap();
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
            let mut mngr = self.chunk_manager.lock().unwrap();
            let dim = self.renderer.lock().unwrap().get_window_size();

            for pos in mngr.visible_chunks.clone() {
                let status = mngr.load_chunk(pos, self.map.clone().unwrap().path).ok();
                if status.is_some() {
                    let (chunk_key, status) = status.unwrap();

                    if let Some(mut chunk) = status.clone().get_chunk().ok() {
                        for (mut unit_pos, mut unit) in chunk.units.clone() {
                            chunk.units.remove_entry(&unit.pos);

                            unit.tick();

                            // For testings : Move unit to mouse cursor
                            // if self.inputs.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
                            // let mouse_pos= self.inputs.mouse_position();
                            //
                            // unit.pos = screen_coords_tile(chunk_key, mouse_pos, self.camera.get_offset(dim));
                            // }
                            //pos

                            chunk.units.insert(unit.pos, unit);
                        }
                        mngr.loaded_chunks.insert(chunk_key, Status::Visible(chunk));

                        // loaded_chunks.insert(pos, status.update_chunk(chunk));
                    }
                }
            }
        }
        // Vérifiez régulièrement si des chunks en `Pending` doivent être relancés
    }

    // Mettre à jour les animations, états visuels, etc.
    pub fn update_visuals(&mut self) {
        if self.map.is_some() {
            let mut mngr = self.chunk_manager.lock().unwrap();
            mngr.visible_chunks = Map::visible_chunks(&self.camera);
            for key in mngr.visible_chunks.clone() {
                // eprintln!("{:?}", key);
                match mngr.load_chunk(key, self.map.clone().unwrap().path) {
                    Ok((key, status)) => {
                        mngr.loaded_chunks.insert(key, status);
                    }
                    Err(_e) => {}
                }
            }
        }
    }
}

use std::{ sync::{ mpsc::{ Receiver, Sender }, Arc, Mutex }, thread };

use chunk::{ thread::Status, Chunk };
use coords::aliases::{ ChunkPos, TilePos };
use map::Map;

use crate::Game;

pub type StatusSender = Arc<Mutex<Sender<Status>>>;
pub type StatusReceiver = Arc<Mutex<Receiver<Status>>>;

/// Threading
///
// ------------------------ //
pub trait BuildThread<Map, StatusSender> {
    fn build_thread(&self, key: (i32, i32));
}
// ------------------------ //

impl BuildThread<Map, StatusSender> for Game {
    fn build_thread(&self, key: (i32, i32)) {
        let seed = match self.map.clone() {
            Some(map) => map.seed,
            _ => panic!("Could not find map seed"),
        };

        let (_height, biome_config) = self.config.clone().biome_from_coord(key);
        let sender = self.sndr.clone();

        Chunk::generate_async(key, seed, &biome_config, sender);
    }
}
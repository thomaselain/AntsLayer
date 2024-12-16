use std::sync::mpsc::{ self, Receiver, RecvTimeoutError, Sender };

use biomes::BiomeConfig;
use chunk::{ thread::{ ChunkKey, Status }, Chunk };
use chunk_manager::{ threads::{ BuildThread, ReceiveStatus }, ChunkManager };

use crate::Map;

pub type MapStatus = (ChunkKey, Status);
pub type MapSender = Sender<MapStatus>;
pub type MapReceiver = Receiver<MapStatus>;
pub struct MapChannel(MapSender, MapReceiver);

impl MapChannel {
    pub fn new() -> Self {
        let (sender, receiver): (MapSender, MapReceiver) = mpsc::channel();
        MapChannel(sender, receiver)
    }
    pub fn sender(&self) -> MapSender {
        self.0.clone()
    }
    pub fn receive(&self) -> Result<MapStatus, RecvTimeoutError> {
        let ((x, y), status) = self.1.recv_timeout(std::time::Duration::new(1, 5_000_000))?;
        Ok(((x, y), status))
    }
}

impl BuildThread<Map, MapSender> for ChunkManager {
    fn build_thread(&self, map: &Map, x: i32, y: i32, sender: MapSender) {
        Chunk::generate_async(x, y, map.seed, BiomeConfig::default(), sender);
    }
}

impl ReceiveStatus<MapChannel> for ChunkManager {
    fn receive_status(&mut self, channel: &MapChannel) -> Option<(ChunkKey, Status)> {
        match channel.receive() {
            Ok((key, status)) => {
                // Traiter le chunk prêt
                if let Status::Ready(chunk) = &status {
                    self.chunks.insert(key, Status::Visible(chunk.clone()));
                } else {
                    self.chunks.insert(key, status.clone());
                }
                Some((key, status))
            }
            Err(e) => {
                // Gestion des erreurs du receiver
                eprintln!("Erreur lors de la réception du status : {:?}", e);
                None
            }
        }
    }
}

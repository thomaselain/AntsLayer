use std::sync::mpsc::{ Receiver, RecvTimeoutError, Sender };

use chunk::thread::{ ChunkKey, Status };

pub type MapChunk = (ChunkKey, Status);
pub type MapSender = Sender<MapChunk>;
pub type MapReceiver = Receiver<MapChunk>;
pub struct MapChannel(MapSender, MapReceiver);

impl MapChannel {
    fn sender(&self) -> MapSender {
        self.0.clone()
    }
    fn receive(&self) -> Result<MapChunk, RecvTimeoutError> {
        let ((x, y), status) = self.1.recv_timeout(std::time::Duration::new(1, 0))?;
        Ok(((x, y), status))
    }
}

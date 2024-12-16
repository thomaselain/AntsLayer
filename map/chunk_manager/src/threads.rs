use chunk::thread::{ChunkKey, Status};

pub trait BuildThread<Map, MapSender> {
    fn build_thread(&self, map: &Map, x: i32, y: i32, sender: MapSender);
}
pub trait ReceiveStatus<MapChannel> {
    fn receive_status(&mut self, channel: &MapChannel) -> Option<(ChunkKey, Status)>;
}
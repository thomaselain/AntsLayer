#[cfg(test)]
use crate::ChunkManager;

#[test]
fn chunk_manager_empty() {
    let chunk_manager = ChunkManager::new();

    assert!(chunk_manager.chunks.is_empty());
}
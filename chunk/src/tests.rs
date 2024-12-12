use super::*;

#[cfg(test)]
fn setup_directory() -> Result<(), std::io::Error> {
    let dir = "test";
    if !Path::new(dir).exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

#[cfg(test)]
fn cleanup_directory() -> Result<(), std::io::Error> {
    let dir = "test";
    if Path::new(dir).exists() {
        fs::remove_dir_all(dir)?;
    }
    Ok(())
}


#[test]
fn chunk_serialization() {
    setup_directory().expect("Failed to set up test directory");

    let original_chunk = Chunk::new();
    let file_path = "test/chunk.bin";

    original_chunk.save(file_path).expect("Failed to save chunk");

    let loaded_chunk = Chunk::load(file_path).expect("Failed to load chunk");
    assert_eq!(original_chunk.tiles, loaded_chunk.get_chunk().expect("Failed to load chunk").tiles);

    cleanup_directory().expect("Failed to clean up test directory");
}

#[test]
fn read_write_chunk() {
    let chunk = Chunk::new();
    let path = "test/chunk.bin";

    chunk.save(path).expect(&format!("Failed to save chunk at {}", path).to_string());

    let loaded_chunk = Chunk::load(path);
    println!("{:?}", loaded_chunk);
}

#[test]
pub fn tile_modification() {
    let mut chunk = Chunk::new();

    let wall_tile = Tile::new((0, 0), TileType::Wall, 1, TileFlags::DIGGABLE);
    chunk.set_tile(0, 0, wall_tile);

    // Sauvegarder le chunk
    chunk.save("test/modifications.bin").expect("Failed to save");

    // Charger le chunk
    let loaded_chunk = Chunk::load("chunk.bin");
    println!("{:?}", loaded_chunk);
}

#[test]
fn chunk_file_operations() {
    let chunk = Chunk::new();
    let file_path = "test/chunk.bin";

    // Test écriture
    chunk.save(file_path).expect("Failed to write chunk");

    // Test lecture
    let chunk_status = Chunk::load(file_path).expect("Failed to read chunk");
    assert_eq!(chunk, chunk_status.get_chunk().expect("Chunk is not ready !"));
}

#[test]
fn skip_in_file() {
    use std::io::Cursor;

    // Créer un fichier virtuel avec deux chunks
    let chunk_data = vec![0u8; CHUNK_SIZE * CHUNK_SIZE * std::mem::size_of::<Tile>()];
    let mut file_data = chunk_data.clone();
    file_data.extend(&chunk_data);

    let mut cursor = Cursor::new(file_data);

    // Vérifie qu'on peut sauter le premier chunk
    Chunk::skip_in_file(&mut cursor).expect("Skip failed");

    // Après avoir sauté, on devrait être à l'offset du deuxième chunk
    assert_eq!(cursor.position(), chunk_data.len() as u64);
}

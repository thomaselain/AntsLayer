use rand::Rng;

use super::*;

#[cfg(test)]
fn setup_test_directory() -> Result<(), std::io::Error> {
    let test_dir = "test";
    if !Path::new(test_dir).exists() {
        fs::create_dir_all(test_dir)?;
    }
    Ok(())
}

#[cfg(test)]
fn cleanup_test_directory() -> Result<(), std::io::Error> {
    let test_dir = "test";
    if Path::new(test_dir).exists() {
        fs::remove_dir_all(test_dir)?;
    }
    Ok(())
}


#[test]
fn test_chunk_serialization() {
    setup_test_directory().expect("Failed to set up test directory");

    let original_chunk = Chunk::new();
    let file_path = "test/test_chunk.bin";

    original_chunk.save(file_path).expect("Failed to save chunk");

    let loaded_chunk = Chunk::load(file_path).expect("Failed to load chunk");
    assert_eq!(original_chunk.tiles, loaded_chunk.tiles);

    cleanup_test_directory().expect("Failed to clean up test directory");
}



#[test]
fn test_read_write_chunk() {
    let mut chunk = Chunk::new();
    let path = "test/chunk.bin";

    chunk.save(path);

    let loaded_chunk = Chunk::load(path);
    println!("{:?}", loaded_chunk);
}

#[test]
pub fn test_tile_modification() {
    let mut chunk = Chunk::new();

    let wall_tile = Tile::new((0, 0), TileType::Wall, 1, TileFlags::DIGGABLE);
    chunk.set_tile(0, 0, wall_tile);

    // Sauvegarder le chunk
    chunk.save("test/modifications.bin").unwrap();

    // Charger le chunk
    let loaded_chunk = Chunk::load("chunk.bin");
    println!("{:?}", loaded_chunk);
}

#[test]
fn test_chunk_file_operations() {
    let chunk = Chunk::new();
    let file_path = "test/chunk.bin";

    // Test écriture
    chunk.save(file_path).expect("Failed to write chunk");

    // Test lecture
    let loaded_chunk = Chunk::load(file_path).expect("Failed to read chunk");
    // assert_eq!(chunk.tiles, loaded_chunk.tiles);
}

#[test]
fn test_skip_in_file() {
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

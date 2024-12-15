use super::*;

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
    let (x, y) = (0, 0);
    let ((x, y), status) = Chunk::generate_default(x, y);
    let chunk = status.get_chunk().expect(&ChunkError::FailedToGenerate.to_string());

    let path = ChunkPath::build("test".to_string(), x, y).expect("Failed to set up test directory");

    chunk.save(path.clone()).expect("Failed to save chunk");

    let ((_x, _y), file_status) = Chunk::load(path.clone()).unwrap();
    assert_eq!(chunk, file_status.get_chunk().ok().unwrap());

    cleanup_directory().expect("Failed to clean up test directory");
}

#[test]
fn read_write_chunk() {
    let path =ChunkPath::build("test".to_string(), 0, 0).expect("Failed to set up test directory");

    let ((_x, _y), status) = Chunk::generate_default(69, 420);

    // Save new chunk
    status
        .clone()
        .get_chunk()
        .expect("Chunk failed to generate")
        .save(path.clone())
        .expect(&format!("Failed to save chunk at {:?}", &path.clone().to_string()));

    println!("Generated chunk : {:?}", status.get_chunk().unwrap());

    let ((_x, _y), loaded_chunk) = Chunk::load(path).unwrap();
    println!("{:?}", loaded_chunk);
}

#[test]
pub fn tile_modification() {
   let path= ChunkPath::build("test".to_string(), 0, 0).expect("Failed to set up test directory");

    let mut chunk = Chunk::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            if x == y {
                let new_tile = Tile::new((0, 0), TileType::Floor, 0, TileFlags::empty());
                chunk.set_tile(x, y, new_tile);
            }
        }
    }

    // Sauvegarder le chunk
    chunk.save(path.clone()).expect("Failed to save");

    // Charger le chunk
    let ((_x, _y), loaded_chunk) = Chunk::load(path.clone()).unwrap();
    println!("{:?}", loaded_chunk);
}

#[test]
fn chunk_file_operations() {
   let path= ChunkPath::build("test/file_operations".to_string(), 0, 0).expect("Failed to set up test directory");
   let chunk = Chunk::new();

    // Test écriture
    chunk.save(path.clone()).expect("Failed to write chunk");

    // Test lecture
    let ((_x, _y), loaded_chunk) = Chunk::load(path.clone()).unwrap();
    assert_eq!(chunk, loaded_chunk.get_chunk().ok().unwrap());
}

#[test]
fn skip_in_file() {
    ChunkPath::build("test".to_string(), 0, 0).expect("Failed to set up test directory");
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

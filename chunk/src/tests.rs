use bincode::Serializer;
use mpsc::Sender;

use super::*;

const TEST_SEED: u32 = 0;

#[test]
fn chunk_serialization() {
    let (x, y) = (0, 0);
    let ((x, y), status) = Chunk::generate_default(x, y);
    let chunk = status.get_chunk().expect(&ChunkError::FailedToGenerate.to_string());

    let path = ChunkPath::build("test".to_string(), x, y).expect("Failed to set up test directory");

    chunk.save(path.clone()).expect("Failed to save chunk");

    println!("{:?}", bincode::serialize(&chunk));
}

#[test]
fn read_write_chunk() {
    let path = ChunkPath::build("test".to_string(), 0, 0).expect("Failed to set up test directory");

    let ((_x, _y), status) = Chunk::generate_default(69, 420);

    // Save new chunk
    status
        .clone()
        .get_chunk()
        .expect("Chunk failed to generate")
        .save(path.clone())
        .expect(&format!("Failed to save chunk at {:?}", &path.clone().to_string()));

    println!("Generated chunk : {:?}", status.get_chunk().unwrap());

    let ((_x, _y), loaded_chunk) = Chunk::load(path, TEST_SEED).unwrap();
    println!("{:?}", loaded_chunk);
}

#[test]
pub fn tile_modification() {
    let path = ChunkPath::build("test".to_string(), 0, 0).expect("Failed to set up test directory");

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
    let ((_x, _y), loaded_chunk) = Chunk::load(path.clone(), TEST_SEED).unwrap();
    println!("{:?}", loaded_chunk);
}

#[test]
fn chunk_file_operations() {
    // Build chunks paths
    let path_1 = ChunkPath::build("test/file_operations".to_string(), 0, 0).expect(
        "Failed to set up test directory"
    );
    let path_2 = ChunkPath::build("test/file_operations".to_string(), 0, 0).expect(
        "Failed to set up test directory"
    );

    // Save chunks
    let chunk_1 = Chunk::new();
    chunk_1.save(path_1.clone()).unwrap();
    let mut chunk_2 = Chunk::new();
    chunk_2.save(path_2.clone()).unwrap();

    // generate new chunk
    let chunk_1 = Chunk::generate_default(path_1.1, path_1.2).1.get_chunk().unwrap();

    // get chunk_2 from file (It should return a Status::ToGenerate)
    let ((_, _), status) = Chunk::load(path_2.clone(), TEST_SEED).unwrap();
    match status {
        Status::ToGenerate => {
            chunk_2 = Chunk::generate_default(path_2.1, path_2.2).1.get_chunk().unwrap();
        }
        _ => {panic!("!")}
    }

    // Test écriture
    chunk_1.save(path_1.clone()).expect("Failed to save chunk");

    // Test lecture
    assert_eq!(Some(chunk_1), Some(chunk_2));
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

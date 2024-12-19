use std::{ sync::mpsc, time::Duration };

use super::*;

#[test]
fn chunk_serialization() {
    let key = (0, 0);

    let (key, status) = Chunk::generate_default(key);
    let chunk = status.get_chunk().expect(&ChunkError::FailedToGenerate.to_string());

    let path = ChunkPath::build("test", key).expect("Failed to set up test directory");

    chunk.save(path.clone()).expect("Failed to save chunk");

    println!("{:?}", bincode::serialize(&chunk));
}

#[test]
fn read_write_chunk() {
    let key = (0, 0);
    let path = ChunkPath::build("test", key).expect("Failed to set up test directory");

    let ((_x, _y), status) = Chunk::generate_default(key);

    // Save new chunk
    status
        .clone()
        .get_chunk()
        .expect("Chunk failed to generate")
        .save(path.clone())
        .expect(&format!("Failed to save chunk at {:?}", &path.clone().to_string()));

    println!("Generated chunk : {:?}", status.get_chunk().unwrap());

    let ((_x, _y), loaded_chunk) = Chunk::default().load(path.0).unwrap();
    println!("{:?}", loaded_chunk);
}

#[test]
pub fn tile_modification() {
    let key = (0, 0);
    let path = ChunkPath::build("test", key).expect("Failed to set up test directory");

    let mut chunk = Chunk::new(key);
    let (_x, _y) = key;
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
    let ((_x, _y), loaded_chunk) = Chunk::new(key).load(path.clone().0).unwrap();
    println!("{:?}", loaded_chunk);
}

#[test]
fn chunk_file_operations() {
    let key = (0, 0);
    // Build chunks paths
    let path_1 = ChunkPath::build("test/file_operations", key).expect(
        "Failed to set up test directory"
    );
    let path_2 = ChunkPath::build("test/file_operations", key).expect(
        "Failed to set up test directory"
    );
    let (sndr, rcvr) = mpsc::channel();
    Chunk::generate_async(key, 0, BiomeConfig::default(), sndr.clone());

    let rc = rcvr.recv_timeout(Duration::from_secs(1)).ok();
    assert_eq!(rc.is_some(), true);

    let chunk = rc.unwrap().1.get_chunk();
    eprintln!("Status received : {:?}", chunk);
    assert_eq!(chunk.is_ok(), true);

    chunk.unwrap().save(path_1.clone()).unwrap();

    let chunk_2 = Chunk::new(key);
    chunk_2.save(path_2.clone()).unwrap();

    // generate new chunk
    let chunk_1 = Chunk::generate_default(path_1.chunk_key()).1.get_chunk().unwrap();

    // Test écriture
    chunk_1.save(path_1.clone()).expect("Failed to save chunk");

    // Test lecture
    assert_eq!(Some(chunk_1), Some(chunk_2));
}

#[test]
fn skip_in_file() {
    let key = (0, 0);
    ChunkPath::build("test", key).expect("Failed to set up test directory");
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

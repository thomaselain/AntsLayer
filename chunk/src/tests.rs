use std::{ sync::mpsc, time::Duration };

use super::*;
#[test]
fn chunk_serialization() {
    const PATH: &str = "test/serialize";
    let key = TilePos::new(0, 0);
    let path = ChunkPath::new(PATH, key);

    let (_, status) = Chunk::generate_default(key);

    let chunk = status.get_chunk();
    assert!(chunk.is_ok());
    let chunk = chunk.unwrap();

    let saved = chunk.save(path.clone());
    assert!(saved.is_ok());

    let deserialize = Chunk::load(path);
    assert!(deserialize.is_ok());
    let _loaded_chunk = deserialize.unwrap();
    // println!("{:?}", _loaded_chunk);

    let serialize = bincode::serialize::<Chunk>(&chunk);
    assert!(serialize.is_ok());
    let _saved_chunk = serialize.unwrap();
    // println!("{:?}", _saved_chunk);
}

#[test]
fn read_write_chunk() {
    const PATH: &str = "test/read_write";
    let key = TilePos::new(0, 0);
    let path = ChunkPath::new(PATH, key);

    let (_key, status) = Chunk::generate_default(key);

    // Save new chunk
    status
        .clone()
        .get_chunk()
        .expect("Chunk failed to generate")
        .save(path.clone())
        .unwrap_or_else(|_| panic!("Failed to save chunk at {:?}", &path.clone().to_string()));

    println!("Generated chunk : {:?}", status.get_chunk().unwrap());

    let (_key, loaded_chunk) = Chunk::load(path.clone()).unwrap();
    println!("{:?}", loaded_chunk);
}

#[test]
pub fn tile_modification() {
    const PATH: &str = "test/tile_modification";

    let key = TilePos::new(0, 0);
    let path = ChunkPath::new(PATH, key);

    let (_, mut chunk) = Chunk::generate_from_biome(key, 0, BiomeConfig::default());

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            if x == y {
                let new_tile = Tile::new(key, TileType::Grass, 0, TileFlags::empty());
                chunk.set_tile(x, y, new_tile);
            }
        }
    }

    // Sauvegarder le chunk
    chunk.save(path.clone()).expect("Failed to save");
    println!("{:?}", chunk);

    // Charger le chunk
    let (_key, loaded_chunk) = Chunk::load(path.clone()).unwrap();
    println!("{:?}", loaded_chunk);
}

#[test]
fn chunk_file_operations() {
    const PATH: &str = "test/file_operations";

    let key = TilePos::new(0, 0);
    // new chunks paths
    let path = ChunkPath::new(PATH, key);

    let (sndr, rcvr) = mpsc::channel();
    Chunk::generate_async(key, 0, BiomeConfig::default(), sndr.clone());

    let mut status = Status::Pending;
    while let Ok((_c, s)) = rcvr.recv_timeout(Duration::from_secs(2)) {
        status = s.clone();
    }

    assert!(status.get_chunk().is_ok());
    let chunk = status.clone().get_chunk().ok();
    assert!(chunk.is_some());
    let chunk = chunk.unwrap();

    // Test écriture
    chunk.save(path.clone()).unwrap();

}

#[test]
fn skip_in_file() {
    let key = TilePos::new(0, 0);
    ChunkPath::new("test", key);
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

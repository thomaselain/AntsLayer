use std::{ sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex }, thread::sleep, time::Duration };

use super::Map;
use chunk::{ threads::{ ChunkKey, Status }, Chunk };
use chunk_manager::ChunkManager;
use tile::{ Tile, TileFlags, TileType };
use crate::{ camera::Camera, renderer::Renderer };

use biomes::{ BiomeConfig, Config };

#[allow(dead_code)]
impl Renderer {
}

#[test]
pub fn map_chunk_loading() {}

#[test]
pub fn map_creation_and_loading() {
    let mut map = Map::new("test/create_load");

    // Générer et sauvegarder un chunk
    let x = 0;
    let y = 0;

    let ((_, _), status) = Chunk::generate_from_biome(x, y, map.seed, &BiomeConfig::default());
    match status {
        Status::Ready(chunk) => {
            map.add_chunk(x, y, chunk);
            map.save().expect("Failed to save map");
            println!("{:?}", chunk);
        }
        _ => panic!(),
    }
}

#[test]
pub fn every_biomes() {
    let config = Config::load().unwrap();

    for biome in config.biomes {
        let path = format!("test/every_biomes_{}", biome.name);
        let map = Map::new(&path);
        let mut chunk_manager = ChunkManager::new();

        let (x, y) = (0, 0);

        let status = chunk_manager.load_chunk(x, y, map.seed);
        'waiting: loop {
            match status {
                Status::Pending => {
                    println!("Waiting for chunk generation ...");
                    sleep(Duration::new(0, 500_000));
                }
                Status::Ready(chunk) => {
                    println!("\nSeed : {} \n Biome {}\n {:?} ", map.seed, biome.name, chunk);
                    break 'waiting;
                }
            }
        }
    }
}

#[test]
pub fn tile_modification() {
    let mut chunk = Chunk::new();
    let mut map = Map::new("test/tile_mod");

    let wall_tile = Tile::new((0, 0), TileType::Wall, 1, TileFlags::DIGGABLE);
    chunk.set_tile(0, 0, wall_tile);

    map.add_chunk(0, 0, chunk);
    // chunk.save(&map.path).expect("Failed to save chunk");

    // Save and wait a little, just in case
    map.save().expect("Failed to save map");
    sleep(Duration::new(1, 0));

    // Charger le chunk
    let loaded_chunk = map.load_chunk(0,0).expect("Failed to load chunk");

    // Vérifier que la tuile a été correctement modifiée
    assert_eq!(
        loaded_chunk.get_tile(0, 0).expect("Tile not found").tile_type,
        TileType::Wall,
        "La tuile n'a pas été correctement modifiée"
    );
    println!("{:?}", chunk);
}

#[test]
fn load_and_generate_chunk() {

}
#[test]
fn dynamic_chunk_loading() {}

#[test]
fn threading() {
    let map = Map::new("test/multi_threading");
    let seed = map.seed;
    let _camera = Camera::new(0.0, 0.0);

    let chunk_manager = Arc::new(Mutex::new(ChunkManager::new()));
    let (sender, receiver): (
        Sender<(ChunkKey, Status)>,
        Receiver<(ChunkKey, Status)>,
    ) = mpsc::channel();

    // Size of the created zone
    let size = 20;
    let range = -size..size;


    println!("Going to generate {} chunks, this may take a while ...", size*size);

    // Lancer les threads de génération
    for x in range.clone() {
        for y in range.clone() {
            Chunk::generate_async(x, y, seed, BiomeConfig::default(), sender.clone());
        }
    }

    let mut chunks = Vec::new();

    // Boucle principale pour surveiller les chunks générés
    'generation: loop {
        if let Ok((key, status)) = receiver.recv_timeout(Duration::from_secs(5)) {
            println!("Chunk {:?} prêt.", key);
            chunks.push(status.clone());

            let mut chunk_manager = chunk_manager.lock().expect("Chunk manager was not ready !");
            chunk_manager.chunks.insert(key, status.clone());

            match status {
                Status::Pending => panic!(),
                Status::Ready(chunk) => {
                    println!("{:?}", chunk);
                    if chunks.len() >= ((size * size) as usize) {
                        break 'generation;
                    }
                }
            }
        } else {
            panic!("Timeout en attente des chunks !");
        }
    }
    println!("{} chunks générés", chunks.len());
    assert_eq!(chunks.len() > 0, true);
}

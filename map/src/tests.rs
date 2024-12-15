use std::{ sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex }, time::Duration };

use super::Map;
use chunk::{ thread::{ ChunkKey, Status }, Chunk };
use chunk_manager::ChunkManager;
use crate::{ camera::Camera, renderer::Renderer, WORLD_STARTING_AREA };

use biomes::{ BiomeConfig, Config };

#[allow(dead_code)]
impl Renderer {}

#[test]
pub fn map_creation_and_loading() {
    let map = Map::new("test_map_loading").unwrap();
    map.save().expect(&format!("Failed to save map at {}", map.path).to_string());
}

#[test]
pub fn every_biomes() {
    let config = Config::load().unwrap();

    for biome in config.biomes {
        let biome_clone = biome.clone();

        let mut map = Map::new(&biome.name).unwrap();
        let (x, y) = (0, 0);

        // let mut chunk_manager = chunk_manager.lock().expect("Failed to lock chunk manager");
        let ((x, y), chunk) = Chunk::generate_from_biome(x, y, map.seed, biome);

        map.add_chunk(x, y, chunk);

        println!(
            "\nSeed : {} \n Biome {}\n {:?}",
            map.seed,
            biome_clone.name,
            map.clone().get_chunk(x, y)
        );
    }
}

#[test]
fn create_and_save() {
    let map = Map::new("save_test");

    let (x, y) = (WORLD_STARTING_AREA * 2, WORLD_STARTING_AREA * 2);
    let chunk = Chunk::new();

    let mut map = map.expect("Map creation failed");
    map.add_chunk(x, y, chunk);
    map.save().expect("Map saving failed");
}

#[test]
#[ignore = "TODO"]
fn map_loading() {}

#[test]
fn threads() {
    let map = Map::new("multi_threading").unwrap();
    let seed = map.seed;
    let _camera = Camera::new(0.0, 0.0);

    let (sender, receiver): (
        Sender<(ChunkKey, Status)>,
        Receiver<(ChunkKey, Status)>,
    ) = mpsc::channel();
    let chunk_manager = Arc::new(Mutex::new(ChunkManager::new()));

    // Size of the created zone
    let size = 20;
    let range = -size..size;

    eprintln!("Going to generate {} chunks, this may take a while ...", size * size);

    // Génération des chunks
    range
        .clone()
        .flat_map(|x| range.clone().map(move |y| (x, y)))
        .collect::<Vec<_>>()
        .iter()
        .for_each(|&(x, y)| {
            Chunk::generate_async(x, y, seed, BiomeConfig::default(), sender.clone());
        });

    let mut chunk_manager = chunk_manager.lock().expect("Chunk manager was not ready !");
    let mut chunks: Vec<Chunk> = Vec::new();

    // Boucle principale pour surveiller les chunks générés
    while let Ok(((x, y), status)) = receiver.recv_timeout(Duration::from_secs(5)) {
        match status.clone().get_chunk() {
            Ok(chunk) => {
                println!("Chunk {:?} prêt.", (x, y));
                chunks.push(chunk);

                println!("{:?}", chunk);
                if chunks.len() >= ((size * size) as usize) {
                    // Generation is done !
                    break;
                }
            }
            Err(status) => {
                match status {
                    Status::ToGenerate => todo!(),
                    Status::Pending => {
                        println!("Attends frero prends ton temps ... ({},{})", x, y);
                    }
                    Status::Visible(_) => todo!(),
                    Status::Ready(_) => todo!(),
                    Status::Error(_) =>todo!(),
                }
            }
        }
    }

    println!("{} chunks générés", chunks.len());
    assert_eq!(chunks.len(), (size * size) as usize);
}

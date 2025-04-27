use std::{ sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex }, time::Duration };

use super::Map;
use chunk::{ thread::Status, Chunk };
use chunk_manager::ChunkManager;
use crate::{ renderer::Renderer, thread::MapStatus, WORLD_STARTING_AREA };

use biomes::Config;

#[allow(dead_code)]
impl Renderer {}

#[test]
pub fn save_load() {
    let saved = Map::new("save_load");
    assert!(saved.is_ok());
    let mut map = saved.unwrap();
    map.add_chunk((0,0).into(), Chunk::default()).unwrap();
    let saved = map.save();
    assert!(saved.is_ok());

    let load = Map::load("save_load");
    assert!(load.is_ok());


}

#[test]
pub fn every_biomes() {
    let config = Config::new();

    for biome in config.biomes {
        let chunk_manager = Arc::new(Mutex::new(ChunkManager::new()));

        let biome_clone = biome.clone();

        let mut map = Map::new(&format!("every_biomes/{}",&biome.name)).unwrap();
        let (sndr, rcvr): (Sender<MapStatus>, Receiver<MapStatus>) = mpsc::channel();
        let key = (0, 0);

        let _chunk_manager = chunk_manager.lock().expect("Failed to lock chunk manager");
        Chunk::generate_async(key, map.seed, &biome, sndr);

        while let Some((key, status)) = rcvr.recv_timeout(Duration::from_secs(5)).ok() {
            match status {
                Status::Pending => {}
                Status::Ready(chunk) => {
                    map.add_chunk(key, chunk).unwrap();
                }
                _ => {
                    panic!("Error");
                }
            }
        }

        println!(
            "\nSeed : {} \n Biome {}\n {:?}",
            map.seed,
            biome_clone.name,
            map.clone().get_chunk(key.into()).ok()
        );
    }
}

#[test]
fn create_and_save() {
    let map = Map::new("save_test");
    assert!(map.is_ok());

    let key = (WORLD_STARTING_AREA, WORLD_STARTING_AREA);
    let chunk = Chunk::default();

    let mut map = map.expect("Map creation failed");
    map.add_chunk(key.into(), chunk).expect("Failed to add chunk to map");
    map.save().expect("Map saving failed");
}

#[test]
#[ignore = "TODO"]
fn map_loading() {}

#[cfg(test)]
mod threads {
    use chunk::Chunk;
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_map_channel() {
        let key = (1, 1);
        let (sndr, rcvr): (Sender<MapStatus>, Receiver<MapStatus>) = mpsc::channel();
        let cfg = Config::default_biome(&Config::new());

        thread::spawn(move || {
            Chunk::generate_async(key, 42, &cfg, sndr.clone());
        });

        while let Some((_key, status)) = rcvr.recv_timeout(Duration::from_secs(2)).ok() {
            match status {
                Status::Ready(chunk) => {
                    eprintln!("{:?}", chunk);
                }
                Status::Pending => println!("Y'a pas le feu au lac, la ..."),
                Status::Error(chunk_error) => {
                    panic!("{:?}", chunk_error);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn array_of_chunks() {
        let map = Map::new("big").unwrap();
        let seed = map.seed;
        let cfg = Config::default_biome(&Config::new());

        let mngr = Arc::new(Mutex::new(ChunkManager::new()));

        // Size of the created zone
        let size = &2;
        let range = (-1i32 * size) / 2..size / 2;

        eprintln!("Going to generate {} chunks, this may take a while ...", size * size);

        // Génération des chunks
        range
            .clone()
            .flat_map(|x| range.clone().map(move |y| (x, y)))
            .collect::<Vec<_>>()
            .iter()
            .for_each(|&(x, y)| {
                let chunk_manager = mngr.lock().unwrap();
                let sndr = chunk_manager.sndr.lock().unwrap().clone();

                Chunk::generate_async(
                    (x, y),
                    seed,
                    &cfg.clone(),
                    sndr.clone()
                );
            });

        let chunk_manager = mngr.lock().expect("Chunk manager was not ready !");
        let mut chunks: Vec<Chunk> = Vec::new();

        // Boucle principale pour surveiller les chunks générés
        while
            let Ok((key, status)) = chunk_manager.rcvr
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(1))
        {
            match status.clone().get_chunk() {
                Ok(chunk) => {
                    println!("Chunk {:?} prêt.", key);
                    chunks.push(chunk.clone());

                    eprintln!("{:?}", chunk);
                }
                Err(status) => {
                    match status {
                        Status::Pending => {
                            eprintln!("Attends frero prends ton temps ... ({:?})", key);
                        }
                        _ => todo!(),
                    }
                }
            }
        }

        eprintln!("{} chunks générés", chunks.len());
        assert_eq!(chunks.len(), (size * size) as usize);
    }
}

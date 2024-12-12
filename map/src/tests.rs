use std::{ sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex }, thread::sleep, time::Duration };

use super::Map;
use chunk::{ threads::{ ChunkKey, Status }, Chunk };
use chunk_manager::{ ChunkManager, Update };
use tile::{ Tile, TileFlags, TileType };
use crate::{ camera::Camera, renderer::Renderer };

use biomes::{ BiomeConfig, Config };

impl Renderer {
    pub fn render_current_chunk(&mut self, map: &Map, camera: &Camera) -> Result<(), String> {
        // Trouver les coordonnées du chunk où est centrée la caméra
        // let current_chunk_x = (camera.coords.x_i32() / (CHUNK_SIZE as i32)) as i32;
        // let current_chunk_y = (camera.coords.y_i32() / (CHUNK_SIZE as i32)) as i32;

        // if let Some(chunk) = map.get_chunk(current_chunk_x, current_chunk_y) {
        //     for y in 0..CHUNK_SIZE {
        //         for x in 0..CHUNK_SIZE {
        //             let tile = chunk.tiles[y][x];
        //             let color = match tile.tile_type {
        //                 TileType::Floor => Color::RGB(100, 200, 100),
        //                 TileType::Liquid => Color::RGB(0, 0, 255),
        //                 TileType::Wall => Color::RGB(150, 150, 150),
        //                 _ => Color::RGB(50, 50, 50),
        //             };

        //             self.canvas.set_draw_color(color);

        //             // Calcul des coordonnées de la tuile
        //             let (rect_x, rect_y) = tile_screen_coords(
        //                 current_chunk_x,
        //                 current_chunk_y,
        //                 x,
        //                 y,
        //                 0,
        //                 0
        //             );

        //             let rect = Rect::new(rect_x, rect_y, TILE_SIZE as u32, TILE_SIZE as u32);
        //             self.canvas.fill_rect(rect)?;
        //         }
        //     }
        // }

        Ok(())
    }

    pub fn render_map(
        &mut self,
        map: &mut Map,
        camera: &Camera,
        chunk_manager: &mut ChunkManager
    ) -> Result<(), String> {
        // let visible_chunks = camera.visible_chunks();
        // let (win_x, win_y) = self.get_window_size();
        // let (offset_x, offset_y) = camera.get_offset(win_x, win_y);

        // map.generate_visible_chunks(camera, chunk_manager);

        // for (x_chunk, y_chunk) in visible_chunks {
        //     if let Some(chunk) = map.get_chunk(x_chunk, y_chunk) {
        //         for y in 0..CHUNK_SIZE {
        //             for x in 0..CHUNK_SIZE {
        //                 let tile = chunk.tiles[y][x];
        //                 let color = match tile.tile_type {
        //                     TileType::Floor => Color::RGB(100, 200, 100),
        //                     TileType::Liquid => Color::RGB(0, 0, 255),
        //                     TileType::Wall => Color::RGB(150, 150, 150),
        //                     _ => Color::RGB(50, 50, 50),
        //                 };

        //                 self.canvas.set_draw_color(color);

        //                 // Calcul des coordonnées du rectangle avec l'offset caméra
        //                 let (rect_x, rect_y) = tile_screen_coords(
        //                     x_chunk,
        //                     y_chunk,
        //                     x,
        //                     y,
        //                     offset_x,
        //                     offset_y
        //                 );

        //                 let rect = Rect::new(
        //                     rect_x,
        //                     rect_y,
        //                     (TILE_SIZE as u32) * (camera.zoom as u32),
        //                     (TILE_SIZE as u32) * (camera.zoom as u32)
        //                 );
        //                 self.canvas.fill_rect(rect)?;
        //             }
        //         }
        //     }
        // }

        Ok(())
    }

    pub fn render_full_map(&mut self, map: &Map) -> Result<(), String> {
        // for (chunk_coords, chunk) in map.get_all_chunks() {
        //     let (x_chunk, y_chunk) = chunk_coords;
        //     for y in 0..CHUNK_SIZE {
        //         for x in 0..CHUNK_SIZE {
        //             let tile = chunk.tiles[y][x];
        //             let color = match tile.tile_type {
        //                 TileType::Floor => Color::RGB(100, 200, 100),
        //                 TileType::Liquid => Color::RGB(0, 0, 255),
        //                 TileType::Wall => Color::RGB(150, 150, 150),
        //                 _ => Color::RGB(50, 50, 50),
        //             };

        //             self.canvas.set_draw_color(color);
        //             let (rect_x, rect_y) = tile_screen_coords(x_chunk, y_chunk, x, y, 0, 0);

        //             let rect = Rect::new(rect_x, rect_y, TILE_SIZE as u32, TILE_SIZE as u32);
        //             self.canvas.fill_rect(rect)?;
        //         }
        //     }
        // }

        Ok(())
    }
}

#[test]
pub fn test_map_creation_and_loading() {
    let path = "test/single_chunk_test.bin";
    let mut map = Map::new(path);
    map.path = path.to_string();

    // Générer et sauvegarder un chunk
    let x = 0;
    let y = 0;

    let ((_, _), status) = Chunk::generate_from_biome(x, y, map.seed, &BiomeConfig::default());
    match status {
        Status::Ready(chunk) => {
            map.add_chunk(x, y, chunk);
            map.save();
            println!("{:?}", chunk);
        }
        _ => panic!(),
    }
}

#[test]
pub fn test_every_biomes() {
    let config = Config::load().unwrap();

    for biome in config.biomes {
        let path = format!("test/every_biomes_{}.bin", biome.name);
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
                    println!("Biome {}, {:?}", biome.name, chunk);
                    break 'waiting;
                }
            }
        }
    }
}

#[test]
pub fn test_tile_modification() {
    let mut chunk = Chunk::new();

    let wall_tile = Tile::new((0, 0), TileType::Wall, 1, TileFlags::DIGGABLE);
    chunk.set_tile(0, 0, wall_tile);

    // Sauvegarder le chunk
    let path = "test/chunk.bin";
    chunk.save(path).expect("Failed to save chunk");

    // Charger le chunk
    let loaded_chunk = Chunk::load(path).expect("Failed to load chunk");

    // Vérifier que la tuile a été correctement modifiée
    let loaded_tile = loaded_chunk.get_tile(0, 0).expect("Tile not found");
    assert_eq!(loaded_tile.tile_type, TileType::Wall, "La tuile n'a pas été correctement modifiée");
    println!("{:?}", chunk);
}

#[test]
fn test_load_and_generate_chunk() {}
#[test]
fn test_dynamic_chunk_loading() {}

#[test]
fn test_threading() {
    let map = Map::new("test/multi_threading.bin");
    let seed = map.seed;
    let camera = Camera::new(0.0, 0.0);

    let chunk_manager = Arc::new(Mutex::new(ChunkManager::new()));
    let (sender, receiver): (
        Sender<(ChunkKey, Status)>,
        Receiver<(ChunkKey, Status)>,
    ) = mpsc::channel();
    let size = 100;
    let range = -size..size;

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

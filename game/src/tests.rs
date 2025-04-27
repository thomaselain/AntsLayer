use coords::aliases::TilePos;
use map::Map;
use unit::Unit;

use crate::Game;

#[test]
// #[ignore = "Runs the whole game"]
pub fn main_test() {
    let mut game = Game::new(sdl2::init().unwrap());
    game.create_world().unwrap();
    add_unit(&mut game);

    game.run();
}

#[allow(dead_code)]
pub fn add_unit(game: &mut Game) {
    // Add unit in middle chunk
    let pos = TilePos::new(0, 0,0);
    game.camera.center_on(0, 0,0);
    game.receive_chunks();
    let unit = Unit::new(pos, 1);

    let mngr = game.chunk_manager.lock().unwrap();
    match mngr.loaded_chunks.get(&pos.into()) {
        Some(status) => {
            // let chunk = status.clone().get_chunk().ok();

        //     if let Some(mut chunk) = chunk {
        //         chunk.units.insert(pos, unit);
        //     }
        }
        None => {
            eprintln!("Chunk not ready yet");
        },
    };
    eprintln!("Unit created at {:?}", unit.pos);
}

#[test]
fn create_map_with_threads() {
    let map = Map::new("map_creation_with_threads").unwrap();
    let mut game = Game::new(sdl2::init().unwrap());
    game.create_world().unwrap();

    game.map = Some(map);

    // game.tick();
    game.map.clone().unwrap().save().unwrap();
    let mut mngr = game.chunk_manager.lock().unwrap();
    while let Ok((key, status)) = game.rcvr.recv_timeout(std::time::Duration::from_secs(2)) {
        mngr.loaded_chunks.insert(key, status);
    }

    for (_key, status) in mngr.loaded_chunks.clone() {
        eprintln!("{:?}", status.get_chunk());
    }
}
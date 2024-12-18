use std::time::Duration;

use map::Map;
use sdl2::libc::sleep;

use crate::game::Game;

#[test]
fn create_map_with_threads() {
    let mut map = Map::new("map_creation_with_threads").unwrap();
    let mut game = Game::new(sdl2::init().unwrap());
    map.create_world(game.sndr.clone()).unwrap();

    game.map = Some(map);

    game.tick();
    game.map.clone().unwrap().save().unwrap();
    let mut mngr = game.chunk_manager.lock().unwrap();
    while let Ok((key, status)) = game.rcvr.recv_timeout(Duration::from_secs(1)) {
        mngr.loaded_chunks.insert(key, status);
    }


    for (key, status) in mngr.loaded_chunks.clone() {
        eprintln!("{:?}", status.get_chunk());
    }
}

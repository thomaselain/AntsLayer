use crate::game;

use {map::Map, game::Game};

#[test]
fn create_map_with_threads() {
    let map = Map::new("map_creation_with_threads").unwrap();
    let mut game = Game::new(sdl2::init().unwrap());
    game.create_world(game.sndr.clone()).unwrap();

    game.map = Some(map);

    game.tick();
    game.map.clone().unwrap().save().unwrap();
    let mut mngr = game.chunk_manager.lock().unwrap();
    while let Ok((key, status)) = game.rcvr.recv_timeout(std::time::Duration::from_secs(1)) {
        mngr.loaded_chunks.insert(key, status);
    }


    for (_key, status) in mngr.loaded_chunks.clone() {
        eprintln!("{:?}", status.get_chunk());
    }
}

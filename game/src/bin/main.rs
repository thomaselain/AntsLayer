use game::Game;
use menu::main_menu;

pub fn main() {
    if main_menu().is_err() {
        todo!("main_menu returned Error!");
    }

    let mut game = Game::new(sdl2::init().unwrap());
    game.create_world(game.sndr.clone()).unwrap();
    game.camera.center_on(0, 0);

    game.run();
}
use game::Game;
use menu::Menu;

pub fn main() {
    let menu = Menu::new();

    let _ = menu.open();

    let mut game = Game::new(sdl2::init().unwrap());
    game.create_world(game.sndr.clone()).unwrap();
    game.camera.center_on(0, 0);

    game.run();
}

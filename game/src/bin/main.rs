use game::Game;

pub fn main() {
    let mut game = Game::new(sdl2::init().unwrap());
    game.create_world().unwrap();
    game.camera.center_on(0, 0);

    game.run();
}
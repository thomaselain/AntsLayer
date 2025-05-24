#[allow(unused_imports)]
use crate::{ chunk::SEA_LEVEL, Game };
#[allow(unused_imports)]
use super::{ Ant, Type };

#[test]
fn joe_the_ant() -> Result<(), ()> {
    let pos = (0, 0, SEA_LEVEL as i32);
    let joe = Ant::new(pos, Type::Explorer);

    let mut game = Game::new(sdl2::init().unwrap());

    game.run();

    Ok(())
}

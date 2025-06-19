#[allow(unused_imports)]
use crate::ant::worker::Worker;
#[allow(unused_imports)]
use crate::ant::ColonyMember;
#[allow(unused_imports)]
use crate::{ chunk::SEA_LEVEL, Game };

#[test]
fn joette_the_ant() -> Result<(), ()> {
    let pos = (0, 0, (SEA_LEVEL as i32) + 10);
    // Joette is born, she's a brave explorer
    // Born in the middle of nowhere,
    // she is seeking adventure and wants to discover the world
    let joette = Worker::new(pos);

    // Game init
    let ttf_context = sdl2::ttf::init().expect("TTF init failed");
    let mut game = Game::new(sdl2::init().unwrap(), &ttf_context);

    // Joette enters the game
    game.ant_manager.colonies[0].ants.insert(0, joette);

    // Game starts
    game.run();

    Ok(())
}

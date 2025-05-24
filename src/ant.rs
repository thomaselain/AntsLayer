pub struct Ant {
    pub pos: (i32, i32, i32),
    pub t: Type,
}

pub enum Type {
    Explorer,
    Fetch,
    Warrior,
}

impl Ant {
    pub fn new(pos: (i32, i32, i32), t: Type) -> Self {
        Self { pos, t }
    }
}
mod test {
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
}

mod test;

mod manager;
/// Name export so it's not confused with Chunk::Manager
pub use manager::Manager as AntManager;

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
    pub fn think(){}
    pub fn act(){}
}
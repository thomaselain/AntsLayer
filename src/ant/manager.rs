use super::Ant;

pub struct Manager {
    ants: Vec<Ant>,
}

impl Manager {
    pub fn new() -> Self {
        Self { ants: vec![] }
    }
}

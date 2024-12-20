use coords::aliases::TilePos;

use crate::Unit;

impl Default for Unit {
    fn default() -> Self {
        let pos = TilePos::new(0, 0);
        let speed = 1;

        Self::new(pos, speed)
    }
}

impl Unit {
    pub fn new(pos: TilePos, speed: u32) -> Self {
        Unit {
            action_dest: TilePos::default(),
            pos,
            speed,
            state: 0, // Tous les bits sont à 0, donc l'unité est inactive
        }
    }

    // Définir ou modifier un état en appliquant un masque
    pub fn set_state(&mut self, mask: u32) {
        self.state |= mask;
    }

    // Retirer un état en appliquant un masque
    pub fn clear_state(&mut self, mask: u32) {
        self.state &= !mask;
    }

    // Vérifier si un état est actif (en utilisant un masque)
    pub fn has_state(&self, mask: u32) -> bool {
        (self.state & mask) != 0
    }

    pub fn tick(&mut self) {
        self.pos += TilePos::new(0, 1);
    }
}

use coords::Coords;

use crate::Unit;

impl Unit {
    pub fn new(coords: Coords<f32>, speed: f32) -> Unit {
        Unit {
            action_dest:None,
            coords,
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
        self.state & mask != 0
    }
}

mod unit;

use coords::Coords;

pub const MOVING: u32 = 1 << 0; // 0001 : L'unité est en mouvement
pub const ATTACKING: u32 = 1 << 1; // 0010 : L'unité est en train d'attaquer
pub const INVISIBLE: u32 = 1 << 2; // 0100 : L'unité est invisible
pub const RESTING: u32 = 1 << 3; // 1000 : L'unité est en train de se reposer


#[allow(dead_code)]
pub struct Unit {
    pub coords: Coords<f32>,
    action_dest: Option<Coords<f32>>, // Destination pour une action
    speed: f32,
    state: u32, // Utilisation d'un masque de 32 bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_creation() {
        let coords = Coords::new(0.0, 0.0);
        let _unit = Unit::new(coords, 1.0);
    }
}

#[cfg(test)]
mod tests_states {
    use super::*;

    #[test]
    fn unit_creation() {
        let coords = Coords::new(0.0, 0.0);
        let  unit = Unit::new(coords, 1.0);

        assert!(!unit.has_state(MOVING)); // Initialement, l'unité n'est pas en mouvement
        assert!(!unit.has_state(ATTACKING)); // L'unité n'attaque pas
    }

    #[test]
    fn set_state() {
        let coords = Coords::new(0.0, 0.0);
        let mut unit = Unit::new(coords, 1.0);

        unit.set_state(MOVING); // On définit l'état de mouvement
        assert!(unit.has_state(MOVING));
    }

    #[test]
    fn clear_state() {
        let coords = Coords::new(0.0, 0.0);
        let mut unit = Unit::new(coords, 1.0);

        unit.set_state(MOVING);
        assert!(unit.has_state(MOVING));

        unit.clear_state(MOVING); // On retire l'état de mouvement
        assert!(!unit.has_state(MOVING));
    }

    #[test]
    fn multiple_states() {
        let coords = Coords::new(0.0, 0.0);
        let mut unit = Unit::new(coords, 1.0);

        unit.set_state(MOVING | ATTACKING); // On définit à la fois les états de mouvement et d'attaque
        assert!(unit.has_state(MOVING));
        assert!(unit.has_state(ATTACKING));
    }
}

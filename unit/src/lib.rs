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

        assert_eq!(unit.has_state(MOVING), false); // Initialement, l'unité n'est pas en mouvement
        assert_eq!(unit.has_state(ATTACKING), false); // L'unité n'attaque pas
    }

    #[test]
    fn set_state() {
        let coords = Coords::new(0.0, 0.0);
        let mut unit = Unit::new(coords, 1.0);

        unit.set_state(MOVING); // On définit l'état de mouvement
        assert_eq!(unit.has_state(MOVING), true);
    }

    #[test]
    fn clear_state() {
        let coords = Coords::new(0.0, 0.0);
        let mut unit = Unit::new(coords, 1.0);

        unit.set_state(MOVING);
        assert_eq!(unit.has_state(MOVING), true);

        unit.clear_state(MOVING); // On retire l'état de mouvement
        assert_eq!(unit.has_state(MOVING), false);
    }

    #[test]
    fn multiple_states() {
        let coords = Coords::new(0.0, 0.0);
        let mut unit = Unit::new(coords, 1.0);

        unit.set_state(MOVING | ATTACKING); // On définit à la fois les états de mouvement et d'attaque
        assert_eq!(unit.has_state(MOVING), true);
        assert_eq!(unit.has_state(ATTACKING), true);
    }
}

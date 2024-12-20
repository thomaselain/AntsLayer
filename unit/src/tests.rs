
#[cfg(test)]
mod tests {
    use coords::aliases::TilePos;
    use crate::Unit;

    const TEST_UNIT_SPEED:u32 = 1;

    #[test]
    fn unit_creation() {
        let coords =TilePos::new(0, 0);
        let _unit = Unit::new(coords, TEST_UNIT_SPEED);
    }
    #[test]
    fn serialize(){
        let unit = Unit::new(TilePos::new(0, 0), TEST_UNIT_SPEED);
    }
}

#[cfg(test)]
mod tests_states {
    use coords::aliases::TilePos;
    use crate::{Unit, ATTACKING, MOVING};
    const TEST_UNIT_SPEED:u32 = 1;

    #[test]
    fn unit_creation() {
        let coords = TilePos::new(0, 0);
        let  unit = Unit::new(coords, TEST_UNIT_SPEED);

        assert!(!unit.has_state(MOVING)); // Initialement, l'unité n'est pas en mouvement
        assert!(!unit.has_state(ATTACKING)); // L'unité n'attaque pas
    }

    #[test]
    fn set_state() {
        let coords = TilePos::new(0, 0);
        let mut unit = Unit::new(coords, TEST_UNIT_SPEED);

        unit.set_state(MOVING); // On définit l'état de mouvement
        assert!(unit.has_state(MOVING));
    }

    #[test]
    fn clear_state() {
        let coords = TilePos::new(0, 0);
        let mut unit = Unit::new(coords, TEST_UNIT_SPEED);

        unit.set_state(MOVING);
        assert!(unit.has_state(MOVING));

        unit.clear_state(MOVING); // On retire l'état de mouvement
        assert!(!unit.has_state(MOVING));
    }

    #[test]
    fn multiple_states() {
        let coords = TilePos::new(0, 0);
        let mut unit = Unit::new(coords, TEST_UNIT_SPEED);

        unit.set_state(MOVING | ATTACKING); // On définit à la fois les états de mouvement et d'attaque
        assert!(unit.has_state(MOVING));
        assert!(unit.has_state(ATTACKING));
    }
}
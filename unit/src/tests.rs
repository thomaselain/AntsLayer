
#[cfg(test)]
mod tests {
    use crate::Unit;

    #[test]
    fn unit_creation() {
        let _unit = Unit::default();
    }
    #[test]
    fn serialize(){
        let unit = Unit::default();
    }
}

#[cfg(test)]
mod tests_states {
    use crate::{Unit, ATTACKING, MOVING};

    #[test]
    fn unit_creation() {
        let  unit = Unit::default();

        assert!(!unit.has_state(MOVING)); // Initialement, l'unité n'est pas en mouvement
        assert!(!unit.has_state(ATTACKING)); // L'unité n'attaque pas
    }

    #[test]
    fn set_state() {
        let mut unit = Unit::default();

        unit.set_state(MOVING); // On définit l'état de mouvement
        assert!(unit.has_state(MOVING));
    }

    #[test]
    fn clear_state() {
        let mut unit = Unit::default();

        unit.set_state(MOVING);
        assert!(unit.has_state(MOVING));

        unit.clear_state(MOVING); // On retire l'état de mouvement
        assert!(!unit.has_state(MOVING));
    }

    #[test]
    fn multiple_states() {
        let mut unit = Unit::default();

        unit.set_state(MOVING | ATTACKING); // On définit à la fois les états de mouvement et d'attaque
        assert!(unit.has_state(MOVING));
        assert!(unit.has_state(ATTACKING));
    }
}
mod unit;

#[cfg(test)]
mod tests;

use coords::aliases::TilePos;
use serde::{Deserialize, Serialize};

pub const MOVING: u32 = 1 << 0; // 0001 : L'unité est en mouvement
pub const ATTACKING: u32 = 1 << 1; // 0010 : L'unité est en train d'attaquer
pub const INVISIBLE: u32 = 1 << 2; // 0100 : L'unité est invisible
pub const RESTING: u32 = 1 << 3; // 1000 : L'unité est en train de se reposer

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct Unit {
    pub pos: TilePos,
    action_dest: TilePos, // Destination pour une action
    speed: u32,
    state: u32, // Utilisation d'un masque de 32 bits
}


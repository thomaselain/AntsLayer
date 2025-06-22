mod joette;
pub mod direction;

pub mod colony;
mod manager;
mod queen;
mod worker;
mod explorer;

mod render;

use std::{ any::Any, time::Instant };

/// Name export so it's not confused with Chunk::Manager
pub use manager::Manager as AntManager;

use crate::{ ant::direction::Direction, chunk::{ tile::TileFlag, ChunkManager } };
#[allow(unused)]
use crate::renderer::{ self, Renderer };

pub trait ColonyMember: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn reset_last_action(&mut self);
    fn pos(&self) -> (i32, i32, i32);
    fn set_pos(&mut self, pos: (i32, i32, i32));

    fn think(&mut self) -> Option<Action>;
    fn render(&self, renderer: &mut Renderer);

    fn walk(&mut self, chunk_mngr: &ChunkManager, direction:Direction);
    fn last_action(&self) -> Instant;
}

fn apply_gravity(pos: &(i32, i32, i32), chunk_mngr: &ChunkManager) -> (i32, i32, i32) {
    let mut current = *pos;

    loop {
        if current.2 == 0 {
            break;
        }

        let below = Direction::Down.add_to(&current);
        if let Some(tile) = chunk_mngr.tile_at(below) {
            if tile.properties.contains(TileFlag::TRAVERSABLE) {
                current = below;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    current
}
pub enum Action {
    Walk(Direction),
    Breed(Vec<Box<dyn ColonyMember>>),
}

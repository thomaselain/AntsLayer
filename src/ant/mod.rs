mod joette;
pub mod direction;

pub mod colony;
mod manager;
mod queen;
mod worker;
mod explorer;

mod render;

use std::any::Any;

/// Name export so it's not confused with Chunk::Manager
pub use manager::Manager as AntManager;

use crate::{ ant::direction::Direction };
#[allow(unused)]
use crate::renderer::{ self, Renderer };

pub trait ColonyMember: Any {
    fn as_any(&self) -> &dyn Any;
    fn new(self, pos: (i32, i32, i32)) -> Box<dyn ColonyMember> where Self: Sized;
    fn render(self, renderer: &mut Renderer) where Self: Sized;

    fn think(&mut self) -> Option<Action>;
   
}
pub enum Action {
    Walk(Direction),
}

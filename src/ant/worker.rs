use std::{ any::Any, time::Instant };

use sdl2::pixels::Color;

use crate::{
    ant::{ direction::Direction, Action, ColonyMember },
    chunk::manager::LoadedChunk,
    renderer::Renderer,
};

#[derive(Clone, Copy)]
pub struct Worker {
    pub pos: (i32, i32, i32),
    pub last_action: Instant,
}

impl ColonyMember for Worker {
    fn new(self, pos: (i32, i32, i32)) -> Box<dyn ColonyMember> {
        Box::new(Self { pos, last_action: Instant::now() })
    }
    fn render(self, renderer: &mut Renderer) {
        println!("Rendering Red ant ! ");
        let (x, y) = (self.pos.0, self.pos.1);
        let (x, y) = renderer.tile_to_screen_coords((x, y));
        renderer.draw_tile((x, y), Color::RED);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn think(&mut self) -> Option<Action> {
        let direction: Direction = rand::random();

        match direction {
            Direction::Up | Direction::Down => {
                return None;
            }
            _ => {
                return Some(Action::Walk(direction));
            }
        }
    }
}

impl Worker {
    pub fn is_in(&self, c: LoadedChunk) -> bool {
        let (x, y, width) = (c.pos.0, c.pos.1, crate::chunk::WIDTH as i32);
        let (x_min, x_max) = (self.pos.0 / width, (self.pos.0 + width) / width - 1);
        let (y_min, y_max) = (self.pos.1 / width, (self.pos.1 + width) / width - 1);

        if x > x_min && x < x_max && y > y_min && y < y_max {
            true
        } else {
            false
        }
    }

    pub fn new(pos: (i32, i32, i32)) -> Box<dyn ColonyMember> {
        Box::new(Self { pos, last_action: Instant::now() })
    }
    pub fn walk(&mut self, d: Direction) {
        self.pos = d.add_to(&self.pos);
        self.last_action = Instant::now();
    }
}

use std::{ any::Any, time::Instant };

use sdl2::pixels::Color;

use crate::{
    ant::{ direction::Direction, Action, ColonyMember },
    chunk::tile::TileFlag,
    renderer::Renderer,
};

#[derive(Clone, Copy)]
pub struct Explorer {
    pub pos: (i32, i32, i32),
    pub last_action: Instant,
}

impl ColonyMember for Explorer {
    fn reset_last_action(&mut self) {
        self.last_action = Instant::now();
    }

    fn last_action(&self) -> Instant {
        self.last_action
    }
    fn pos(&self) -> (i32, i32, i32) {
        self.pos
    }
    fn set_pos(&mut self, pos: (i32, i32, i32)) {
        self.pos = pos;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn render(&self, renderer: &mut Renderer) {
        let (x, y, z) = self.pos;

        if z > renderer.camera.2 {
            return;
        }
        let (x, y) = renderer.tile_to_screen_coords((x, y));
        renderer.draw_tile((x, y), Color::RGB(200, 0, 0));
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

    fn walk(&mut self, chunk_mngr: &crate::chunk::ChunkManager, direction: Direction) {
        let dest = direction.add_to(&self.pos());
    
        if let Some(tile) = chunk_mngr.tile_at(dest) {
            if tile.properties.contains(TileFlag::TRAVERSABLE) {
                self.set_pos(dest);
                println!("An explorer is walking to {:?}!", direction);
            } else {
                let climb_dest = Direction::Up.add_to(&dest);
                if let Some(climb_tile) = chunk_mngr.tile_at(climb_dest) {
                    if climb_tile.properties.contains(TileFlag::TRAVERSABLE) {
                        self.set_pos(climb_dest); // 🧗 destination corrigée
                        println!("An explorer is climbing !");
                    }
                }
            }
        }
    }
    
}

impl Explorer {
    pub fn new(pos: (i32, i32, i32)) -> Box<dyn ColonyMember> {
        Box::new(Self { pos, last_action: Instant::now() })
    }
}

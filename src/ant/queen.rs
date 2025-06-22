#[allow(unused)]
use std::{ any::Any, time::{ Duration, Instant } };

use sdl2::pixels::Color;

#[allow(unused)]
use crate::ant::{ colony::Colony, direction::Direction, Action };
#[allow(unused)]
use crate::{
    ant::{ explorer::Explorer, worker::Worker, AntManager, ColonyMember },
    chunk::{ Chunk, ChunkManager },
    renderer::Renderer,
};

#[derive(Clone, Copy)]
pub struct Egg {
    pub hatch: fn(pos: (i32, i32, i32)) -> Box<dyn ColonyMember>,
}
impl Egg {}

#[derive(Clone)]
pub struct Queen {
    pub pos: (i32, i32, i32),
    pub last_action: Instant,
    pub eggs: Vec<Egg>,
}

impl ColonyMember for Queen where Self: Sized {
    fn last_action(&self) -> Instant {
        self.last_action
    }
    fn reset_last_action(&mut self) {
        self.last_action = Instant::now();
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn pos(&self) -> (i32, i32, i32) {
        self.pos
    }
    fn set_pos(&mut self, pos: (i32, i32, i32)) {
        self.pos = pos;
    }
    fn render(&self, renderer: &mut Renderer) {
        let (x, y, z) = self.pos;

        if z > renderer.camera.2 {
            return;
        }
        let (x, y) = renderer.tile_to_screen_coords((x, y));
        renderer.draw_tile((x, y), Color::YELLOW);
    }
    fn think(&mut self) -> Option<Action> {
        if self.eggs.len() > 0 {
            println!(
                "The queen is giving birth, {:?} new ants arrived in this world !",
                self.eggs.len()
            );
            let newborns = self.breed();
            Some(Action::Breed(newborns))
        } else {
            None
        }
    }

    #[allow(unused)]
    fn walk(&mut self, chunk_mngr: &ChunkManager, direction: Direction) {
        panic!("Why would i go anywhere ?")
    }
}

impl Queen {
    pub const BREEDING_TIMER: f32 = 2.0;
    pub fn breed(&mut self) -> Vec<Box<dyn ColonyMember>> where Self: Sized {
        let mut newborns = vec![];

        for egg in self.eggs.clone() {
            newborns.push((egg.hatch)(self.pos));
            self.eggs.pop();
        }

        newborns
    }
    pub fn new_worker(&mut self) {
        self.eggs.insert(0, Egg { hatch: Worker::new });
    }
}

#[test]
fn queen() {
    let mut ant_manager = AntManager::new();
    let mut chunk_manager = ChunkManager::empty();
    let chunk = Chunk::generate((0, 0), &chunk_manager.world_noise);
    let chunk = chunk.join().unwrap();
    chunk_manager.loaded_chunks.insert((0, 0), chunk.clone());

    let bok: &mut Colony = &mut ant_manager.colonies[Colony::PLAYER];

    // println!("{:?}", self.queen.last_action().duration_since(last_tick));
    if bok.queen.last_action().duration_since(Instant::now()) > Duration::from_millis(1000) {
        if let Some(action) = bok.queen.think() {
            match action {
                Action::Walk(_) => {
                    panic!("Why would the queen go anywhere ?");
                }
                Action::Breed(mut newborns) => {
                    bok.ants.append(&mut newborns);
                    bok.queen.reset_last_action();
                }
            }
        }
    } else {
        panic!("The queen should have something to do !");
    }
}

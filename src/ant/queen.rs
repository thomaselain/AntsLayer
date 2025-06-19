use std::{ any::Any, time::{ Duration, Instant } };

use sdl2::pixels::Color;

use crate::ant::{ colony::Colony, Action };
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
    fn new(self, pos: (i32, i32, i32)) -> Box<dyn ColonyMember> {
        Box::new(Self { pos, last_action: Instant::now(), eggs: vec![] })
    }
    fn render(self, renderer: &mut Renderer) {
        if self.pos.2 != renderer.camera.2 {
            return;
        }
        let (x, y) = (self.pos.0, self.pos.1);
        let (x, y) = renderer.tile_to_screen_coords((x, y));
        renderer.draw_tile((x, y), Color::YELLOW);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn think(&mut self) -> Option<Action> {
        if Instant::now().duration_since(self.last_action) > Duration::from_secs_f32(1.0) {
            println!(
                "The queen is giving birth, {:?} new ants arrived in this world !",
                self.eggs.len()
            );
            self.clone().breed();
            self.last_action = Instant::now();
        }

        None
    }
}

impl Queen {
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

impl Colony {
    pub fn babies(&mut self, babies: Vec<Box<dyn ColonyMember + 'static>>) -> Result<(), ()> {
        for baby in babies {
            // todo!("Change 0 to find the colony");
            self.ants.push(baby);
        }
        Ok(())
    }
}
#[test]
fn queen() {
    let pos = (0, 0, 0);

    let mut queen = Queen {
        pos,
        last_action: Instant::now(),
        eggs: vec![Egg { hatch: Explorer::new }],
    };
    assert!(queen.eggs.len() == 1);
    queen.new_worker();
    assert!(queen.eggs.len() == 2);

    let mut ant_manager = AntManager::new();
    let mut chunk_manager = ChunkManager::empty();
    let chunk = Chunk::generate((0, 0), &chunk_manager.world_noise);
    let chunk = chunk.join().unwrap();
    chunk_manager.loaded_chunks.insert((0, 0), chunk.clone());

    let newborns = queen.breed();

    let bok: &mut Colony = &mut ant_manager.colonies[Colony::PLAYER];

    bok.babies(newborns);

    queen.think();
}

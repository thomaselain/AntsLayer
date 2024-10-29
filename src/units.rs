use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use rand;
use rand::seq::SliceRandom;

use terrain::Terrain;

use crate::{
    coords::{self},
    camera::{self},
    terrain::{self, TileType},
};
use coords::Coords as Coords;

const UNIT_SIZE: u32 = terrain::TILE_SIZE;

#[derive(Copy, Clone)]
pub enum RaceType {
    HUMAN,
    ANT,
    ALIEN,
}

#[derive(Copy, Clone)]
pub enum JobType {
    MINER,
    FARMER,
    FIGHTER,
    BUILDER,
}

#[derive(Copy, Clone, Debug)]
pub enum ActionType {
    WANDER,
    WAIT,
    MOVE,
    DIG,
    EAT,
    SLEEP,
    FIGHT,
    BUILD,
}

#[doc = "Unit.speed is thinking speed (in milliseconds) not moving speed"]
#[derive(Clone)]
pub struct Unit {
    color: Color,
    pub job: JobType,
    pub race: RaceType,
    pub coords: Coords,
    pub action_coords: Option<Coords>,
    pub action_queue: Vec<ActionType>,
    speed: i32,
    last_action_timer: i32,
}

fn random_direction() -> i32 {
    let choices = [-1, 0, 1];
    *choices.choose(&mut rand::thread_rng()).unwrap()
}

impl Unit {
    pub fn new(race: RaceType, job: JobType, coords: Coords) -> Unit {
        Unit {
            color: match race {
                RaceType::HUMAN => Color::BLUE,
                RaceType::ALIEN => Color::GREEN,
                RaceType::ANT => Color::RED,
                _ => {
                    panic!("Invalid RaceType for new Unit !!!")
                }
            },
            race,
            job,
            coords,
            action_coords: None,
            action_queue: vec![],
            last_action_timer: 0,
            speed: match race {
                RaceType::HUMAN => 300,
                RaceType::ANT => 10,
                RaceType::ALIEN => 50,
                _ => {
                    panic!("Invalid RaceType for new Unit !!!")
                }
            },
        }
    }
}

pub trait Actions {
    fn do_action(&mut self, terrain: Terrain, action: ActionType);
    fn think(&mut self, terrain: Terrain, delta_time: i32);
    fn draw_at(&self, canvas: &mut Canvas<Window>, zoom: f32) -> Result<(), String>;
    fn r#move(&mut self, terrain: Terrain, m: Coords);
    fn dig(&self);
    fn build(&self);
}

impl Actions for Unit {
    fn do_action(&mut self, terrain: Terrain, action: ActionType) {
        match action {
            ActionType::MOVE => {
                self.r#move(terrain, Coords { x: 1, y: 0 });
            }
            ActionType::WANDER => {
                self.r#move(
                    terrain,
                    Coords {
                        x: random_direction(),
                        y: random_direction(),
                    },
                );
            }
            _ => {}
        }
    }
    /// Decide what to do next
    fn think(&mut self, terrain: Terrain, delta_time: i32) {
        if self.last_action_timer >= self.speed {
            if let Some(action) = self.action_queue.first() {
                self.do_action(terrain, *action);
                self.last_action_timer = 0;
                self.action_queue.remove(0);
            }
        }
        if self.action_queue.is_empty() {
            self.action_queue.push(ActionType::WANDER);
        }
        self.last_action_timer += delta_time;
    }
    /// Draw unit at the correct coords on sdl_window
    fn draw_at(&self, canvas: &mut Canvas<Window>, zoom: f32) -> Result<(), String> {
        let x: f32 = self.coords.x as f32 * zoom;
        let y: f32 = self.coords.y as f32 * zoom;

        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(
            x as i32,
            y as i32,
            UNIT_SIZE as u32,
            UNIT_SIZE as u32,
        ))?;
        Ok(())
    }

    //move
    fn r#move(&mut self, terrain: Terrain, m: Coords) {
        let target_x = (self.coords.x + m.x * UNIT_SIZE as i32) as usize;
        let target_y = (self.coords.y + m.y * UNIT_SIZE as i32) as usize;

        if terrain.data.len() > target_x
            && terrain.data[target_x].len() > target_y
            && terrain.data[target_x][target_y] == TileType::AIR
        {
            self.coords.x += m.x as i32;
            self.coords.y += m.y as i32;
        }
    }
    fn dig(&self) {}
    fn build(&self) {}
}

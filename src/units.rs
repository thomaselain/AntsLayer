use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use rand;
use rand::seq::SliceRandom;

const UNIT_SIZE: i32 = 5;

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

#[derive(Copy, Clone, Debug)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

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
        //pub fn new(&self, race: RaceType, job: JobType, coords: (i32, i32)) -> Unit {
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
                // Thinking speed, not real speed (value is in milliseconds, the higher the slower they act)
                RaceType::HUMAN => 300,
                RaceType::ANT => 10,
                RaceType::ALIEN => 150,
                _ => {
                    panic!("Invalid RaceType for new Unit !!!")
                }
            },
        }
    }
}

pub trait Actions {
    fn do_action(&mut self, action: ActionType);
    fn think(&mut self, delta_time: i32);
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn r#move(&mut self, m: Coords);
    fn dig(&self);
    fn build(&self);
}

impl Actions for Unit {
    fn do_action(&mut self, action: ActionType) {
        match action {
            ActionType::MOVE => {
                self.r#move(Coords { x: 1, y: 0 });
            }
            ActionType::WANDER => {
                self.r#move(Coords {
                    x: random_direction(),
                    y: random_direction(),
                });
            }
            _ => {}
        }
    }
    // Decide what to do next
    fn think(&mut self, delta_time: i32) {
        if self.last_action_timer >= self.speed {
            if let Some(action) = self.action_queue.first() {
                self.do_action(*action);
                self.last_action_timer = 0;
                self.action_queue.remove(0);
            }
        } else {
            self.last_action_timer += delta_time;
            self.action_queue.push(ActionType::WANDER);
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.coords.x, self.coords.y, UNIT_SIZE as u32, UNIT_SIZE as u32))?;
        Ok(())
    }

    //move
    fn r#move(&mut self, m: Coords) {
        self.coords.x += m.x * UNIT_SIZE;
        self.coords.y += m.y * UNIT_SIZE;
    }
    fn dig(&self) {}
    fn build(&self) {}
}

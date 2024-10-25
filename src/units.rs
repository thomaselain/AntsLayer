use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use rand;

const UNIT_SIZE: i32 = 2;

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
    job: JobType,
    race: RaceType,
    pub coords: Coords,
    pub action_queue: Vec<ActionType>,
    speed: i32,
    last_action_timer: i32,
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
            action_queue: vec![],
            last_action_timer: 0,
            speed: match race {
                RaceType::HUMAN => 1500,
                RaceType::ANT => 500,
                RaceType::ALIEN => 500,
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
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.coords.x, self.coords.y, 10, 10))?;
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

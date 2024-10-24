use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

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
                RaceType::HUMAN => 1000,
                RaceType::ALIEN => 1000,
                RaceType::ANT => 1000,
                _ => {
                    panic!("Invalid RaceType for new Unit !!!")
                }
            },
        }
    }
}

pub trait Actions {
    fn do_action(&mut self);
    fn think(&mut self, delta_time: i32);
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn r#move(&mut self, m: Coords);
    fn dig(&self);
    fn build(&self);
}

impl Actions for Unit {
    fn do_action(&mut self) {
        if let Some(action) = self.action_queue.first() {
            match action {
                ActionType::MOVE => {
                    self.r#move(Coords { x: 1, y: 1 });
                }
                _ => {}
            }
            self.action_queue.remove(0);
        }
    }

    // Decide what to do next
    fn think(&mut self, delta_time: i32) {
        self.last_action_timer += delta_time;

        if self.last_action_timer >= self.speed {
            self.last_action_timer = 0;
            self.action_queue.push(ActionType::MOVE);
            return;
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.coords.x, self.coords.y, 10, 10))?;
        Ok(())
    }
    //move
    fn r#move(&mut self, m: Coords) {
        self.coords.x += m.x;
        self.coords.y += m.y;
    }
    fn dig(&self) {}
    fn build(&self) {}
}

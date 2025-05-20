use sdl2::{ event::Event, keyboard::Keycode, libc::exit, sys::KeyCode };

use crate::{ renderer::Renderer, Game };

pub enum Direction {
    Up,
    Down,
    North,
    East,
    South,
    West,
}

pub trait ToDirection {
    fn to_direction(self) -> Result<Direction, Keycode>;
}

/// Movement inputs
/// C => Down
/// W => Up
/// Z => North
/// S => South
/// Q => West
/// D => East
impl ToDirection for Keycode {
    fn to_direction(self) -> Result<Direction, Keycode> {
        match self {
            Keycode::C => Ok(Direction::Down),
            Keycode::W => Ok(Direction::Up),
            Keycode::Z => Ok(Direction::North),
            Keycode::S => Ok(Direction::South),
            Keycode::Q => Ok(Direction::West),
            Keycode::D => Ok(Direction::East),

            key => { Err(key) }
        }
    }
}

// Inputs are all handled thanks to this struct
#[derive(Clone)]
pub struct Inputs {
    // A vector of all the current unprocessed queued pressed keys
    pub pressed_keys: Vec<Keycode>,
    // Same but for inputs that are not meant to be repeated
    pub just_pressed_keys: Vec<Keycode>,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            pressed_keys: Vec::new(),
            just_pressed_keys: Vec::new(),
            // mouse_pressed: Vec::new(),
            // mouse_position: (0, 0),
            // wheel_dir: 0,
        }
    }

    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn is_key_just_pressed(&self, key: Keycode) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    // pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
    //     self.mouse_pressed.contains(&button)
    // }

    // pub fn mouse_position(&self) -> (i32, i32) {
    //     self.mouse_position
    // }
}

impl Game {
    pub fn process_input(&mut self) -> Result<(), ()> {
        for event in self.events.drain(..) {
            if let Event::KeyDown { keycode: Some(key), repeat, .. } = event {
                // Directional inputs
                if let Ok(dir) = key.to_direction() {
                    match dir {
                        // C W
                        Direction::Up | Direction::Down if !repeat => {
                            self.renderer.move_camera(dir);
                        }

                        // Z Q S D
                        Direction::North | Direction::East | Direction::South | Direction::West => {
                            self.renderer.move_camera(dir);
                        }
                        _ => {}
                    }
                }

                // EXIT GAME (at next loop)
                match key {
                    Keycode::ESCAPE => {
                        self.running = false;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl Renderer {
    pub fn move_camera(&mut self, dir: Direction) {
        let c = self.camera;
        let mv = match dir {
            Direction::Up => (0, 0, 1),
            Direction::Down => (0, 0, -1),
            Direction::North => (0, 1, 0),
            Direction::East => (-1, 0, 0),
            Direction::South => (0, -1, 0),
            Direction::West => (1, 0, 0),
        };

        self.camera = (c.0 + mv.0, c.1 + mv.1, c.2 + mv.2);
    }
}

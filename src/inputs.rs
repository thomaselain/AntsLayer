use sdl2::{ event::Event, keyboard::Keycode };

use crate::{
    ant::Direction,
    interface::{ self },
    Game,
};

pub trait ToDirection {
    fn to_direction(self) -> Result<Direction, Keycode>;
}

/// Movement inputs (AZERTY)
/// A => Decrease camera range
/// E => Increase camera range
/// C => Down
/// W => Up
/// Z => North
/// S => South
/// Q => West
/// D => East
impl ToDirection for Keycode {
    fn to_direction(self) -> Result<Direction, Keycode> {
        match self {
            Keycode::C => Ok(Direction::Up),
            Keycode::W => Ok(Direction::Down),
            Keycode::Z => Ok(Direction::North),
            Keycode::S => Ok(Direction::South),
            Keycode::Q => Ok(Direction::West),
            Keycode::D => Ok(Direction::East),

            key => Err(key),
        }
    }
}

// Inputs are all handled thanks to this struct
// #[derive(Clone)]
pub struct Inputs {
    // A vector of all the current unprocessed queued pressed keys
    pub pressed_keys: Vec<Keycode>,
    // Same but for inputs that are not meant to be repeated
    pub just_pressed_keys: Vec<Keycode>,
    // save slider id when dragged
    pub dragging_slider_id: Option<interface::Id>,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            pressed_keys: Vec::new(),
            just_pressed_keys: Vec::new(),
            dragging_slider_id: None,
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

impl<'ttf> Game<'ttf> {
    pub fn process_input(&mut self) -> Result<(), ()> {
        for event in self.events.drain(..) {
            ///////////////////////////
            //      Interface
            // (Sliders and buttons)
            ///////////////////////////
            match event {
                Event::MouseButtonDown { x, y, .. } => {
                    // Find wich slider is clicked on, if any
                    self.inputs.dragging_slider_id = self.interface.check_sliders_at((x, y));

                    for (_label, button) in &mut self.interface.buttons {
                        button.handle_click(&mut self.renderer, x, y)?;
                    }
                }
                Event::MouseButtonUp { .. } => {
                    // Reset all sliders on mouse release
                    self.interface.clear_sliders_state();
                    self.inputs.dragging_slider_id = None;
                }
                Event::MouseMotion { x, .. } => {
                    //
                    if let Some(id) = self.inputs.dragging_slider_id {
                        let slider_value = self.interface.update(id, x);
                        match id {
                            interface::Id::Zoom => {
                                self.renderer.tile_size = slider_value as usize;
                            }
                            interface::Id::CameraZ => {
                                self.renderer.camera.2 = slider_value;
                            }
                            //
                            _ => {}
                        }
                    }
                }
                _ => {}
            }

            /////////////////
            // Keyboard
            /////////////////
            if let Event::KeyDown { keycode: Some(key), repeat: _, .. } = event {
                // Directional inputs
                if let Ok(dir) = key.to_direction() {
                    match dir {
                        // C W
                        // Direction::Up | Direction::Down if !repeat => {
                        Direction::Up | Direction::Down => {
                            self.renderer.move_camera(dir);
                        }

                        // Z Q S D
                        Direction::North | Direction::East | Direction::South | Direction::West => {
                            self.renderer.move_camera(dir);
                        }
                    }
                }
                // EXIT GAME (at next loop)
                match key {
                    Keycode::G => {
                        self.renderer.is_grid_enabled = if self.renderer.is_grid_enabled {
                            false
                        } else {
                            true
                        };
                    }
                    Keycode::A => {
                        self.renderer.decrease_view_dist().unwrap();
                    }
                    Keycode::E => {
                        self.renderer.increase_view_dist().unwrap();
                    }
                    Keycode::ESCAPE => {
                        self.ant_manager.ants.clear();
                        self.chunk_manager.loaded_chunks.clear();
                        self.running = false;
                    }
                    _ => {}
                }
            }

            /////////////////
            // Left click
            /////////////////
            // if let Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } = event {
            // }

            /////////////////
            // Mouse Wheel
            /////////////////
            if let Event::MouseWheel { y, direction, .. } = event {
                let scroll = match direction {
                    sdl2::mouse::MouseWheelDirection::Normal => y,
                    sdl2::mouse::MouseWheelDirection::Flipped => -y,
                    _ => 0,
                };

                if scroll > 0 {
                    self.renderer.tile_size += 1;
                } else if scroll < 0 {
                    self.renderer.tile_size = self.renderer.tile_size - 1;
                }
                self.renderer.tile_size = self.renderer.tile_size.clamp(
                    crate::renderer::MIN_TILE_SIZE,
                    crate::renderer::MAX_TILE_SIZE
                );
            }
        }
        Ok(())
    }
}

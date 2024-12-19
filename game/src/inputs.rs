use map::Directions;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::game::Game;

pub trait ToDirection {
    fn to_direction(self) -> Result<Directions, Keycode>;
}
impl ToDirection for Keycode {
    fn to_direction(self) -> Result<Directions, Keycode> {
        match self {
            Keycode::Z => Ok(Directions::North), // Haut
            Keycode::S => Ok(Directions::South), // Bas
            Keycode::Q => Ok(Directions::West), // Gauche
            Keycode::D => Ok(Directions::East), // Droite

            key => { Err(key) }
        }
    }
}

#[derive(Clone)]
pub struct Inputs {
    pub key_pressed: Vec<Keycode>, // Clés actuellement enfoncées
    mouse_pressed: Vec<MouseButton>, // Boutons de la souris enfoncés
    mouse_position: (i32, i32), // Position de la souris
    wheel_dir: i32,
}

impl Default for Inputs {
    fn default() -> Self {
        Self::new()
    }
}

impl Inputs {
    // Crée un nouveau gestionnaire d'entrées
    pub fn new() -> Inputs {
        Inputs {
            key_pressed: Vec::new(),
            mouse_pressed: Vec::new(),
            mouse_position: (0, 0),
            wheel_dir: 0,
        }
    }

    // Met à jour l'état des entrées en fonction des événements
    pub fn update(&mut self, events: &[Event]) {
        self.key_pressed.clear();
        self.mouse_pressed.clear();

        for event in events {
            match event {
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        if !self.is_key_pressed(*key) {
                            self.key_pressed.push(*key);
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        if let Some(pos) = self.key_pressed.iter().position(|x| x == key) {
                            self.key_pressed.remove(pos);
                        }
                    }
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    self.mouse_pressed.push(*mouse_btn);
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if let Some(pos) = self.mouse_pressed.iter().position(|x| x == mouse_btn) {
                        self.mouse_pressed.remove(pos);
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_position = (*x, *y);
                }
                Event::MouseWheel { y, .. } => {
                    self.wheel_dir = *y;
                }
                _ => {}
            }
        }
    }

    // Vérifie si une touche spécifique est enfoncée
    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.key_pressed.contains(&key)
    }

    // Vérifie si un bouton de souris est enfoncé
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }

    // Retourne la position de la souris
    pub fn mouse_position(&self) -> (i32, i32) {
        self.mouse_position
    }
}

impl Game {
    // Gérer les événements (clavier, souris, etc.)
    pub fn process_input(&mut self) -> Result<(), ()> {
        self.inputs.update(&self.events);

        if self.inputs.is_key_pressed(Keycode::KP_MINUS) && self.camera.speed > 0.1 {
            self.camera.speed -= 0.01;
            println!("Camera speed set to {}", self.camera.speed);
        } else if self.inputs.is_key_pressed(Keycode::KP_PLUS) {
            self.camera.speed += 0.01;
            println!("Camera speed set to {}", self.camera.speed);
        } else if self.inputs.is_key_pressed(Keycode::A) && self.camera.render_distance > 1 {
            self.camera.render_distance -= 1;
            println!("Camera render distance set to {}", self.camera.render_distance);
        } else if self.inputs.is_key_pressed(Keycode::E) {
            self.camera.render_distance += 1;
            println!("Camera render distance set to {}", self.camera.render_distance);
        }
        if self.inputs.is_key_pressed(Keycode::N) {
            self.current_biome = if self.current_biome + 1 < self.config.biomes.len() {
                self.current_biome + 1
            } else {
                0
            };
            println!("{:?}", self.config.biomes[self.current_biome].name);
            self.create_world(self.sndr.clone()).unwrap();
        }
        if self.inputs.is_key_pressed(Keycode::R) {
            self.create_world(self.sndr.clone()).unwrap();
        }
        if self.inputs.is_key_pressed(Keycode::L) {
            // self.load_world()?;
        }

        if &self.inputs.wheel_dir > &0 {
            self.camera.zoom *= 1.1;
            eprintln!("WHEEL UP : zoom set to :{}", self.camera.zoom);
            self.inputs.wheel_dir = 0;
        } else if &self.inputs.wheel_dir < &0 {
            self.camera.zoom /= 1.1;
            eprintln!("WHEEL DOWN : zoom set to :{}", self.camera.zoom);
            self.inputs.wheel_dir = 0;
        }
        if self.map.is_some() && self.inputs.is_key_pressed(Keycode::Space) {
            self.map.as_ref().unwrap().save().expect("Failed to save map");
            println!("Map saved !");
        }

        if let Some(key) = self.inputs.key_pressed.last() {
            if let Ok(dir) = key.to_direction() {
                self.camera.move_dir(dir);
                self.inputs.key_pressed.pop();
            }
        }
        // self.inputs.clear();
        Ok(())
    }
}

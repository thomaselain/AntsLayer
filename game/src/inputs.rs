use map::Directions;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

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
pub struct Inputs {
    pub key_pressed: Vec<Keycode>, // Clés actuellement enfoncées
    mouse_pressed: Vec<MouseButton>, // Boutons de la souris enfoncés
    mouse_position: (i32, i32), // Position de la souris
}

impl Inputs {
    // Crée un nouveau gestionnaire d'entrées
    pub fn new() -> Inputs {
        Inputs {
            key_pressed: Vec::new(),
            mouse_pressed: Vec::new(),
            mouse_position: (0, 0),
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
                        if !self.is_key_pressed(*key){
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

mod button;
mod map_editor;

use std::collections::HashMap;

use button::{ Button, MenuError, MenuLabel, Output };
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{ EventPump, Sdl };

type ButtonList = HashMap<u8, Button>;

pub struct Menu {
    buttons: ButtonList,
    sdl: Sdl,
    canvas: Canvas<Window>,
}

impl Menu {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("Menu Principal", 600, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Menu { buttons: Menu::build_buttons(), sdl, canvas }
    }

    fn build_buttons() -> ButtonList {
        let mut buttons = ButtonList::new();

        buttons.insert(1, Button::new(MenuLabel::MapEditor));
        buttons.insert(2, Button::new(MenuLabel::LoadWorld));
        buttons.insert(3, Button::new(MenuLabel::Settings));
        buttons.insert(4, Button::new(MenuLabel::Exit));
        // buttons.insert(5, Button::new(MenuLabel::MainMenu)); // Current menu

        buttons
    }

    fn catch_menu_events(&self, mut events: EventPump) -> Result<MenuLabel, ()> {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Ok(MenuLabel::Exit); // Quitter sur escape
                }
                Event::MouseButtonDown { x, y, .. } => {
                    for (_id, button) in &self.buttons {
                        if button.is_clicked(x, y) {
                            // button.do_button_stuff; ???

                            match button.label {
                                MenuLabel::MapEditor => {
                                    return Ok(MenuLabel::MapEditor);
                                }
                                MenuLabel::LoadWorld => {
                                    return Ok(MenuLabel::LoadWorld);
                                }
                                MenuLabel::Settings => {
                                    todo!("Ouvrir les paramètres"); // Paramètres
                                }
                                MenuLabel::Exit => {
                                    return Ok(MenuLabel::Exit);
                                }
                                MenuLabel::MainMenu => todo!(),
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Err(())
    }
    pub fn main(&self) -> Result<MenuLabel, MenuError> {
        loop {
            let events = self.sdl.event_pump().unwrap();
            if let Ok(next_menu) = self.catch_menu_events(events) {
                return Ok(next_menu);
            }else{

            }
        }
    }

    pub fn open(&mut self) -> Result<MenuLabel, MenuError> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0)); // Fond noir
        self.canvas.clear();

        for (_id, button) in &self.buttons {
            button.render(&mut self.canvas);
        }

        self.canvas.present();

        self.main()

        // Err(MenuError::InvalidOutput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_editor() {
        let mut menu = Menu::new();
        let next_menu = menu.open();

        println!("Next menu : {:#?}",next_menu);
        assert!(next_menu.is_ok());
        let label = next_menu.unwrap();

        assert_eq!(label, MenuLabel::MapEditor);
        menu.map_editor();
    }
}

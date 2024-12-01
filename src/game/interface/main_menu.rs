use coords::Coords;
use sdl2::{ event::Event, keyboard::Keycode, mouse::MouseState, pixels::Color, rect::Rect };

use crate::game::render::window::{ self, init_sdl2_window, Renderer };

use super::{ Button, ButtonId };
pub fn create_main_menu_buttons() -> Vec<Button> {
    let mut buttons = Vec::new();

    buttons.push(Button::new(ButtonId::Start));
    buttons.push(Button::new(ButtonId::Settings));
    buttons.push(Button::new(ButtonId::Exit));
    buttons
}
pub fn main_menu() -> Result<(), ()> {
    let buttons: Vec<Button> = create_main_menu_buttons();
    let (sdl_context, win) = init_sdl2_window();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut renderer = Renderer::new(
        (sdl_context, win),
        window::WIDTH as usize,
        window::HEIGHT as usize
    );

    'menu: loop {
        // let mouse_state = MouseState::new(&event_pump);

        for b in buttons.iter() {
            b.clone().draw(&mut renderer);
        }

        for event in event_pump.poll_iter() {
            // let mouse_x = mouse_state.x();
            // let mouse_y = mouse_state.y();

            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Err(());
                }

                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        for b in buttons.iter() {
                            match b.clone().clicked(Coords(x, y)) {
                                Ok(button_id) => {
                                    match button_id {
                                        ButtonId::Exit => {
                                            // Go back one menu
                                            return Err(());
                                        }
                                        ButtonId::Start => {
                                            // Start game
                                            return Ok(());
                                        }
                                        ButtonId::Settings => todo!("Open settings window"),
                                    }
                                }
                                Err(_) => {
                                    // No action
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        renderer.canvas.present();
    }
}

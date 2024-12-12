mod button;

use button::Button;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main_menu() -> Result<(), ()> {
    // Initialisation SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Menu Principal", 600, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Créer les boutons du menu
    let buttons: Vec<Button> = vec![
        Button::new("Gérer la map", 300, 150, 200, 50, Color::GRAY),
        Button::new("Lancer le jeu", 300, 250, 200, 50, Color::BLUE),
        Button::new("Paramètres", 300, 350, 200, 50, Color::GREEN),
        Button::new("Quitter", 300, 450, 200, 50, Color::RED)
    ];

    // Boucle principale
    'main_menu: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'main_menu;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    for button in &buttons {
                        if button.is_clicked(x, y) {
                            match button.label.as_str() {
                                "Gérer la map" => {
                                    todo!("Outil de création de maps");
                                }
                                "Lancer le jeu" => {
                                    return Ok(());
                                } // Lancer le jeu
                                "Paramètres" => todo!("Ouvrir les paramètres"), // Paramètres
                                "Quitter" => {
                                    return Err(());
                                } // Quitter l'application
                                _ => {}
                            }
                            return Ok(());
                        }
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_menu; // Quitter sur escape
                }
                _ => {}
            }
        }

        // Rendu du menu
        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Fond noir
        canvas.clear();

        for button in &buttons {
            button.render(&mut canvas);
        }

        canvas.present();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main_menu();
        print!("yep");
    }
}

use game::game::Game;
use sdl2::{ keyboard::Keycode, pixels::Color };
use menu::main_menu;

pub fn main() {
    game();
}

#[test]
pub fn main_test() {
    game();
}

pub fn game() {
    if main_menu().is_err() {
    todo!("main_menu returned Error!");
    }

    let mut game = Game::new(sdl2::init().unwrap());

    // Boucle de jeu
    'running: loop {
        let mut event_pump = game.sdl.event_pump().unwrap();

        if let Ok(mut renderer) = game.renderer.lock() {
            renderer.canvas.clear();
            renderer.canvas.set_draw_color(Color::RGB(0, 0, 0));
        } else {
            panic!();
        }

        if game.inputs.is_key_pressed(Keycode::Escape) {
            break 'running;
        }

        for event in event_pump.poll_iter() {
            game.events.push(event);
        }
        game.tick(); // Dessiner la carte et les unités
        if let Ok(mut renderer) = game.renderer.lock() {
            renderer.canvas.present();
        } else {
            panic!();
        }
    }
}

use std::{ process::{ ExitCode, Termination }, time::{ Duration, Instant } };

use chunk::{ biomes::Params, manager::Manager };
use inputs::Inputs;
use renderer::Renderer;
use sdl2::{ event::Event, pixels::Color, ttf::Sdl2TtfContext, Sdl };

mod debug;

mod chunk;
mod inputs;
mod renderer;

pub struct Game {
    // Game engine
    pub running: bool,
    pub last_tick: Instant,
    pub tick_rate: Duration,

    // Chunk
    pub chunk_manager: Manager,
    pub renderer: Renderer,

    // SDL2 fields
    pub sdl: Sdl,
    pub ttf_context: Sdl2TtfContext,
    pub events: Vec<Event>,
    pub inputs: Inputs,
}

#[cfg(test)]
mod tests {
    use crate::main;
    use crate::Game;

    #[test]
    fn test_main() -> Result<Game, ()> {
        // Run the game and get its Result
        let game = main();

        Ok(game?)
    }
}

impl Game {
    pub fn new(sdl: Sdl) -> Game {
        let renderer = Renderer::new(&sdl, "Ants Layer", 800, 600).expect(
            "Failed to create game renderer"
        );
        let ttf_context = sdl2::ttf
            ::init()
            .map_err(|e| e.to_string())
            .expect("Failed to init SDL2::ttf");

        Game {
            running: true,
            last_tick: Instant::now(),
            tick_rate: Duration::from_secs_f64(1.0 / 60.0),

            chunk_manager: Manager::new(),
            renderer,

            sdl,
            ttf_context,
            events: Vec::new(),
            inputs: Inputs::new(),
        }
    }
}

impl Game {
    pub fn create_world(&mut self) -> Result<(), ()> {
        self.chunk_manager = Manager::new();

        Ok(())
    }

    pub fn tick(&mut self) {
        if self.process_input().is_err() {
            todo!("Invalid input handling");
        }

        self.render();
    }

    fn render(&mut self) {
        for ((x, y), c) in &self.chunk_manager.loaded_chunks {
            c.draw(&mut self.renderer, (x, y));
        }

        // Top-left info display
        //
        // camera coords
        self.display_debug().unwrap();
    }

    pub fn run(&mut self) {
        // Boucle de jeu
        while self.running {
            let mut event_pump = self.sdl.event_pump().unwrap();

            // Clear screen at the start of each frame
            // (could be improved a lot)
            self.renderer.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.renderer.canvas.clear();

            for event in event_pump.poll_iter() {
                self.events.push(event);
            }

            self.tick();

            self.renderer.canvas.present();
        }
    }
}

impl Termination for Game {
    fn report(self) -> std::process::ExitCode {
        // Testing stuff when Game exits
        // self.chunk_manager.loaded_chunks.clear();

        if self.chunk_manager.loaded_chunks.is_empty() {
            ExitCode::SUCCESS
        } else {
            eprintln!("Game stoped with chunks still active");
            ExitCode::FAILURE
        }
    }
}

fn main() -> Result<Game, ()> {
    let sdl = sdl2::init().expect("Failed to init SDL2");
    let mut game = Game::new(sdl);

    game.run();
    
    Ok(game)
}

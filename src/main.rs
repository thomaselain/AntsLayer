use std::{ ops::Range, time::{ Duration, Instant } };

use ant::{ AntManager };
use chunk::{ ChunkManager, CHUNK_WIDTH };
use inputs::Inputs;
use rand::distributions::uniform::SampleUniform;
use renderer::{ Renderer, VIEW_DISTANCE };
use sdl2::{ event::Event, pixels::Color, ttf::Sdl2TtfContext, Sdl };

//  ------
mod debug;
//  ------

// Chunks
mod chunk;
// Units
mod ant;

// Game engine
mod inputs;
mod renderer;

pub struct Game {
    // Game engine
    pub running: bool,
    pub last_tick: Instant,
    pub first_tick: Instant,
    pub tick_rate: Duration,

    // Chunk
    pub ant_manager: AntManager,
    pub chunk_manager: ChunkManager,
    pub renderer: Renderer,

    // SDL2 fields
    pub sdl: Sdl,
    pub ttf_context: Sdl2TtfContext,
    pub events: Vec<Event>,
    pub inputs: Inputs,
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::{ main, ant::{ Ant, Type }, Game };

    #[test]
    #[ignore = "test doesn't wait for the game to exit so it shows as FAILED"]
    fn test_main() -> Result<(), ()> {
        // Run the game and get its Result
        main()?;
        Ok(())
    }
}

impl Game {
    // Seconds that happends since the game started
    fn elapsed_secs(&self) -> f64 {
        self.first_tick.clone().elapsed().as_secs_f64()
    }

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
            first_tick: Instant::now(),
            tick_rate: Duration::from_secs_f64(1.0 / 60.0),

            ant_manager: AntManager::new(),
            chunk_manager: ChunkManager::new(),
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
        self.chunk_manager = ChunkManager::new();

        Ok(())
    }

    pub fn tick(&mut self) {
        // Let the ants think !
        self.ant_manager.tick(&self.chunk_manager, self.last_tick);

        if self.process_input().is_err() {
            todo!("Invalid input handling");
        }
    }

    fn render(&mut self) {
        let timestamp = self.elapsed_secs();
        
        let (x_min, x_max, y_min, y_max) = (
            (self.renderer.camera.0 - VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
            (self.renderer.camera.0 + VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
            (self.renderer.camera.1 - VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
            (self.renderer.camera.1 + VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
        );

        self.chunk_manager.render(&mut self.renderer, timestamp);

        self.ant_manager.render(&mut self.renderer);

        // Top-left info display
        #[cfg(test)]
        {
            self.display_debug().unwrap();
        }
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

            // Maybe multithread will be needed for chunks rendering
            self.render();

            self.renderer.canvas.present();
        }
    }
}

fn main() -> Result<(), ()> {
    let mut game = Game::new(sdl2::init().unwrap());

    game.run();

    eprintln!("Active ants   : {:?}", game.ant_manager.ants.len());
    eprintln!("Active chunks : {:?}", game.chunk_manager.loaded_chunks.len());

    Ok(())
}

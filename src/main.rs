use std::{ process::{ ExitCode, Termination }, time::{ Duration, Instant } };

use ant::{ AntManager };
use chunk::{ ChunkManager, CHUNK_WIDTH };
use inputs::Inputs;
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
        if self.process_input().is_err() {
            todo!("Invalid input handling");
        }

        self.render();
    }

    fn render(&mut self) {
        let timestamp = self.elapsed_secs();
        let (x_min, x_max, y_min, y_max) = (
            (self.renderer.camera.0 - VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
            (self.renderer.camera.0 + VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
            (self.renderer.camera.1 - VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
            (self.renderer.camera.1 + VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
        );

        for chunk in &self.chunk_manager.loaded_chunks {
            chunk.c.draw(&mut self.renderer, chunk.pos, timestamp);
        }

        // Top-left info display
        #[cfg(test)]
        {
            self.display_debug(format!("Camera pos : {:?}", self.renderer.camera), 1).unwrap();
            self.display_debug(format!("Time       : {:.2?}", self.elapsed_secs()), 2).unwrap();
            self.display_debug(format!("Tile size  : {:?}", self.renderer.tile_size), 3).unwrap();
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
    let mut game = Game::new(sdl2::init().unwrap());

    game.run();

    Ok(game)
}

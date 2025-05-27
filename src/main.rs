use std::{ time::{ Duration, Instant } };

use ant::{ AntManager };
use chunk::{ ChunkManager };
use debug_interface::Interface;
use inputs::Inputs;
use renderer::{ Renderer };
use sdl2::{ event::Event, pixels::Color, ttf::Sdl2TtfContext, Sdl };

//  ------
mod debug;
mod debug_interface;
//  ------

// Chunks
mod chunk;
// Units
mod ant;

// Game engine
mod inputs;
mod renderer;

pub struct Game {
    // Engine
    pub running: bool,
    pub debug_interface: Interface,

    // Frames and Ticks tracking
    pub tps: u32,
    pub fps: u32,
    pub ticks: u32,
    pub frames: u32,
    pub last_frame: Instant,
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
    use crate::{ main, ant::{ Ant, Type }, chunk::generation::SEA_LEVEL, Game };

    #[test]
    #[ignore = "Cannot init SDL on more than one thread"]
    fn test_main() -> Result<(), ()> {
        let mut game = Game::new(sdl2::init().unwrap());
        game.run();

        // Joette
        let mut ants = vec![Ant::new((0, 0, SEA_LEVEL as i32), Type::Explorer)];

        for i in 10..20 {
            ants.push(Ant::new((i, i, SEA_LEVEL as i32), Type::Fetcher));
        }

        // Check if ants are copied correctly
        assert!(game.ant_manager.ants.is_empty());
        game.ant_manager.ants = ants;

        Ok(())
    }
}

impl Game {
    // Seconds that happends since the game started
    fn elapsed_secs(&self) -> f64 {
        self.first_tick.clone().elapsed().as_secs_f64()
    }

    pub fn new(sdl: Sdl) -> Game {
        let renderer = Renderer::new(&sdl, "Ants Layer").expect(
            "Failed to create game renderer"
        );
        let ttf_context = sdl2::ttf
            ::init()
            .map_err(|e| e.to_string())
            .expect("Failed to init SDL2::ttf");

        Game {
            running: true,
            debug_interface:Interface::new(),

            tps: Default::default(),
            fps: Default::default(),
            ticks: Default::default(),
            frames: Default::default(),

            last_frame: Instant::now(),
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
        #[cfg(test)]
        self.update_tps();

        // Let the ants think !
        self.ant_manager.tick(&self.chunk_manager, self.last_tick);

        if self.process_input().is_err() {
            todo!("Invalid input handling");
        }
    }
    #[allow(unused)]
    fn update_fps(&mut self) {
        if Instant::now().duration_since(self.last_frame) < Duration::from_secs(1) {
            // Increment frames Per Second if less than a sec happened
            self.frames += 1;
        } else {
            // Otherwise, update counter
            self.fps = self.frames;
            self.frames = 0;
            self.last_frame = Instant::now();
        }
    }
    #[allow(unused)]
    fn update_tps(&mut self) {
        if Instant::now().duration_since(self.last_tick) < Duration::from_secs(1) {
            // Increment frames Per Second if less than a sec happened
            self.ticks += 1;
        } else {
            // Otherwise, update counter
            self.tps = self.ticks;
            self.ticks = 0;
            self.last_tick = Instant::now();
        }
    }
    fn render(&mut self) {
        let timestamp = self.elapsed_secs();

        #[cfg(test)]
        self.update_fps();

        // Render distance calucation
        // let (x_min, x_max, y_min, y_max) = (
        // (self.renderer.camera.0 - VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
        // (self.renderer.camera.0 + VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
        // (self.renderer.camera.1 - VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
        // (self.renderer.camera.1 + VIEW_DISTANCE) / (CHUNK_WIDTH as i32),
        // );

        self.chunk_manager.render(&mut self.renderer, self.ant_manager.ants.clone(), timestamp);

        // Top-left info display
        #[cfg(test)]
        {
            self.display_info().unwrap();
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
            self.renderer.update_window_size();
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

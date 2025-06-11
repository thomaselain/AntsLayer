use std::{ sync::{ Arc, Mutex }, time::{ Duration, Instant } };

use ant::{ AntManager };
use chunk::{ ChunkManager };
use inputs::Inputs;
use interface::Interface;
use renderer::{ Renderer };
use sdl2::{ event::Event, pixels::Color, ttf::Sdl2TtfContext, Sdl };

//  ------
mod debug;
mod interface;
//  ------

mod time;
// Chunks
mod chunk;
// Units
mod ant;

// Game engine
mod inputs;
mod renderer;

pub struct Game<'ttf> {
    // Engine
    pub running: bool,
    pub interface: Interface,

    // Frames and Ticks tracking
    pub tps: Arc<Mutex<u64>>,
    pub fps: Arc<Mutex<u32>>,
    pub ticks: u64,
    pub frames: u32,
    pub last_frame: Instant,
    pub last_tick: Instant,
    pub first_tick: Instant,
    pub tick_rate: Duration,

    // Chunk
    pub ant_manager: AntManager,
    pub chunk_manager: ChunkManager,
    pub renderer: Renderer<'ttf>,

    // SDL2 fields
    pub sdl: Sdl,
    pub events: Vec<Event>,
    pub inputs: Inputs,
}

#[cfg(test)]
mod tests {
    use crate::chunk::SEA_LEVEL;
    #[allow(unused_imports)]
    use crate::{ main, ant::{ Ant, Type }, Game };

    #[test]
    #[ignore = "Cannot init SDL on more than one thread"]
    fn test_main() -> Result<(), ()> {
        let ttf_context = sdl2::ttf::init().expect("TTF init failed");
        let mut game = Game::new(sdl2::init().unwrap(), &ttf_context);
        game.run();

        // Joette
        let mut ants = vec![Ant::new((0, 0, SEA_LEVEL as i32), Type::Explorer)];

        for i in 10..20 {
            ants.push(Ant::new((i, i, SEA_LEVEL as i32), Type::Fetcher));
        }

        Ok(())
    }
}

impl<'ttf> Game<'ttf> {
    pub fn new(sdl: Sdl, ttf_context: &'ttf Sdl2TtfContext) -> Game<'ttf> {
        let renderer = Renderer::new(&sdl, &ttf_context, "Ants Layer").expect(
            "Failed to create game renderer"
        );

        Game {
            running: true,
            interface: Interface::new(),

            tps: Default::default(),
            fps: Default::default(),
            ticks: Default::default(),
            frames: Default::default(),

            last_frame: Instant::now(),
            last_tick: Instant::now(),
            first_tick: Instant::now(),
            tick_rate: Duration::from_secs_f64(1.0 / 60.0),

            ant_manager: AntManager::new(),
            chunk_manager: ChunkManager::default(),
            renderer,

            sdl,
            events: Vec::new(),
            inputs: Inputs::new(),
        }
    }
}

impl<'ttf> Game<'ttf> {
    // Seconds that happends since the game started
    fn elapsed_secs(&self) -> f64 {
        self.first_tick.clone().elapsed().as_secs_f64()
    }
}

impl<'ttf> Game<'ttf> {
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
            if let Some(mut fps) = self.fps.lock().ok() {
                *fps = self.frames;
                self.frames = 0;
                self.last_frame = Instant::now();
            }
        }
    }
    #[allow(unused)]
    fn update_tps(&mut self) {
        if Instant::now().duration_since(self.last_tick) < Duration::from_secs(1) {
            // Increment frames Per Second if less than a sec happened
            self.ticks += 1;
        } else {
            // Otherwise, update counter
            if let Some(mut tps) = self.tps.lock().ok() {
                *tps = self.ticks;
                self.ticks = 0;
                self.last_tick = Instant::now();
            }
        }
    }

    fn render(&mut self) {
        let timestamp = self.elapsed_secs();
        // let dims = self.renderer.dims;

        #[cfg(test)]
        self.update_fps();

        self.chunk_manager.look_for_new_chunks(&mut self.renderer);

        self.chunk_manager.render(&mut self.renderer, &self.ant_manager, timestamp);

        // Top-left info display
        #[cfg(test)]
        {
            self.display_info().unwrap();
        }

        self.interface.render(&mut self.renderer);
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
    let ttf_context = sdl2::ttf::init().expect("TTF init failed");
    let mut game = Game::new(sdl2::init().unwrap(), &ttf_context);

    game.run();

    eprintln!("Active ants   : {:?}", game.ant_manager.ants.len());
    eprintln!("Active chunks : {:?}", game.chunk_manager.loaded_chunks.len());

    Ok(())
}

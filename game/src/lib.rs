mod inputs;

pub mod game;
pub mod update;

#[cfg(test)]
pub mod tests;

use std::{ sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex }, time::{ Duration, Instant } };

#[allow(unused_imports)]
use chunk_manager::Draw;
#[allow(unused_imports)]
use chunk_manager::DrawAll;
use chunk_manager::ChunkManager;

use biomes::Config;
use inputs::Inputs;
use sdl2::{ event::Event, Sdl };
use map::{ camera::Camera, renderer::Renderer, thread::MapStatus, Map };

#[allow(unused_imports)]
use unit::{ Unit, MOVING };

pub struct Game {
    pub chunk_manager: Arc<Mutex<ChunkManager>>,
    pub renderer: Arc<Mutex<Renderer>>,
    pub last_tick: Instant,
    pub tick_rate: Duration, // Pour contrôler la fréquence des ticks (30 ou 60 fps)

    pub config: Config,
    pub current_biome: usize, // TMP FOR TESTING BIOMES

    pub camera: Camera,
    pub sdl: Sdl,
    pub events: Vec<Event>,
    pub inputs: Inputs,

    pub sndr: Sender<MapStatus>,
    pub rcvr: Receiver<MapStatus>,

    pub map: Option<Map>,
}

impl Game {
    pub fn new(sdl: Sdl) -> Game {
        let renderer = Renderer::new(
            &sdl,
            "Ants Layer",
            game::WIN_DEFAULT_WIDTH,
            game::WIN_DEFAULT_HEIGHT
        ).expect("Failed to create game renderer");
        let config = Config::new();
        let camera = Camera::new(0.0, 0.0,0.0);
        let (sndr, rcvr) = mpsc::channel();

        Game {
            last_tick: Instant::now(),
            tick_rate: Duration::from_secs_f64(1.0 / 60.0), // 60 FPS par défaut
            chunk_manager: Arc::new(Mutex::new(ChunkManager::new())),
            renderer: Arc::new(Mutex::new(renderer)),

            sndr,
            rcvr,

            config,
            current_biome: 0,

            camera,
            sdl,
            events: Vec::new(),
            inputs: Inputs::new(),
            map: None,
        }
    }
}

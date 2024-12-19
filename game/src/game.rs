use std::{ sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex }, time::{ Duration, Instant } };

use chunk::{ thread::{ ChunkKey, Status }, Chunk, ChunkPath };
#[allow(unused_imports)]
use chunk_manager::Draw;
#[allow(unused_imports)]
use chunk_manager::DrawAll;
use chunk_manager::ChunkManager;

use biomes::{ BiomeConfig, Config };
use sdl2::{ event::Event, keyboard::Keycode, Sdl };
use map::{ camera::Camera, renderer::Renderer, thread::MapStatus, Map, WORLD_STARTING_AREA };

#[allow(unused_imports)]
use unit::{ Unit, MOVING };

use crate::inputs::{ Inputs, ToDirection };
pub const WIN_DEFAULT_WIDTH: u32 = 1000;
pub const WIN_DEFAULT_HEIGHT: u32 = 800;

pub struct Game {
    pub chunk_manager: Arc<Mutex<ChunkManager>>,
    pub renderer: Arc<Mutex<Renderer>>,
    pub last_tick: Instant,
    pub tick_rate: Duration, // Pour contrôler la fréquence des ticks (30 ou 60 fps)

    pub config: Config,
    pub current_biome: usize,// TMP FOR TESTING BIOMES

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
            WIN_DEFAULT_WIDTH,
            WIN_DEFAULT_HEIGHT
        ).expect("Failed to create game renderer");
        let config = Config::new();
        let camera = Camera::new(0.0, 0.0);
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

    // pub fn load_world(&mut self) -> Result<Map, ()> {
    //     let path = "./data/test_world";
    //     println!("Loading {}", path);
    //     // Ok(Map::load(path).expect(&format!("Failed to load map at '{}'", path)))
    // }

    pub fn create_world(&mut self, sndr: Sender<(ChunkKey, Status)>) -> Result<(), String> {
        self.map = Some(Map::new("default").unwrap());
        let half_size = WORLD_STARTING_AREA / 2;

        for x in -half_size..=half_size {
            for y in -half_size..=half_size {
                Chunk::generate_async(
                    (x, y),
                    self.map.clone().unwrap().seed,
                    self.config.biomes[self.current_biome].clone(),
                    sndr.clone()
                );
            }
        }

        Ok(())
    }

    pub fn tick(&mut self) {
        // 1. Gérer les entrées utilisateur
        if self.process_input().is_err() {
            todo!("Invalid input handling");
        }

        // 2. Mettre à jour la logique du jeu (mouvement, IA, gestion des ressources, etc.)
        self.update_game_logic();

        // 3. Mettre à jour les animations et états visuels
        self.update_visuals();

        // 4. Rendu graphique
        self.render();
    }

    // Mettre à jour les unités, la carte, les ressources, etc.
    fn update_game_logic(&mut self) {
        if self.map.is_some() {
            let mut mngr = self.chunk_manager.lock().unwrap();

            // Vérifiez les chunks reçus via le receiver
            while let Ok((key, status)) = self.rcvr.recv_timeout(self.tick_rate) {
                match status {
                    Status::Ready(chunk) | Status::Visible(chunk) => {
                        let mut map = self.map.clone().unwrap();
                        chunk.save(ChunkPath::build(&map.path, key).unwrap()).unwrap();
                        mngr.loaded_chunks.insert(key, status.clone());
                        map.add_chunk(key, chunk).unwrap();
                    }
                    Status::Pending => {}
                    _ => {
                        eprintln!("Statut inconnu pour le chunk {:?}: {:?}", key, status);
                    }
                }
            }
        }
        // Vérifiez régulièrement si des chunks en `Pending` doivent être relancés
    }

    // Mettre à jour les animations, états visuels, etc.
    fn update_visuals(&mut self) {
        if self.map.is_some() {
            let mut mngr = self.chunk_manager.lock().unwrap();
            mngr.visible_chunks = Map::visible_chunks(&self.camera);
            for key in mngr.visible_chunks.clone() {
                // eprintln!("{:?}", key);
                match mngr.load_chunk(key, self.map.clone().unwrap().path) {
                    Ok((key, status)) => {
                        mngr.loaded_chunks.insert(key, status);
                    }
                    Err(_e) => {}
                }
            }
        }
    }

    fn render(&mut self) {
        if self.map.is_some() {
            let chunk_manager = self.chunk_manager.lock().unwrap();
            let mut renderer = self.renderer.lock().unwrap();
            // chunk_manager.draw_all(&mut self.map.clone().unwrap(), &mut renderer, &self.camera);
            chunk_manager.draw(&mut renderer, &self.camera);
        }
    }
}

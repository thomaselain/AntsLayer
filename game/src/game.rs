use std::{
    collections::HashMap,
    io::{ self, Write },
    sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex },
    time::{ Duration, Instant },
};

use chunk::{ thread::{ ChunkKey, Status }, Chunk };
#[allow(unused_imports)]
use chunk_manager::Draw;
#[allow(unused_imports)]
use chunk_manager::DrawAll;
use chunk_manager::ChunkManager;
use chunk_manager::Update;

use biomes::BiomeConfig;
use sdl2::{ event::Event, keyboard::Keycode, Sdl };
use map::{ camera::Camera, renderer::Renderer, thread::MapStatus, Map };

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

    pub biome_config: BiomeConfig,
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
        let biome_config = BiomeConfig::default();
        let camera = Camera::new(0.0, 0.0);
        let (sndr, rcvr) = mpsc::channel();

        Game {
            last_tick: Instant::now(),
            tick_rate: Duration::new(1, 0) / 60, // 60 FPS par défaut
            chunk_manager: Arc::new(Mutex::new(ChunkManager::new())),
            renderer: Arc::new(Mutex::new(renderer)),

            sndr,
            rcvr,

            biome_config,
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

    pub fn create_world(&mut self) -> Result<(), ()> {
        if self.map.is_none() {
            // Demander le nom du fichier de la carte à l'utilisateur
            print!("Entrez le nom du fichier de la carte : ");
            io::stdout().flush().unwrap(); // Assure-toi que le prompt est affiché avant de lire l'entrée.

            // Créer le chemin du fichier
            let file_name = if cfg!(test) == false {
                "test".to_string()
            } else {
                let mut file_name = String::new();
                io::stdin().read_line(&mut file_name).unwrap();
                file_name.trim().to_string() // Supprimer les espaces inutiles
            };

            // Charger la carte avec le fichier donné

            self.map = Some(Map::new(&file_name)).expect("Failed to create map").ok();
            Ok(())
        } else {
            Err(())
        }
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

    // Gérer les événements (clavier, souris, etc.)
    fn process_input(&mut self) -> Result<(), ()> {
        self.inputs.update(&self.events);

        if self.inputs.is_key_pressed(Keycode::KP_MINUS) && self.camera.speed > 0.1 {
            self.camera.speed -= 0.01;
            println!("Camera speed set to {}", self.camera.speed);
        } else if self.inputs.is_key_pressed(Keycode::KP_PLUS) {
            self.camera.speed += 0.01;
            println!("Camera speed set to {}", self.camera.speed);
        } else if self.inputs.is_key_pressed(Keycode::A) && self.camera.render_distance > 1 {
            self.camera.render_distance -= 1;
            println!("Camera zoom set to {}", self.camera.render_distance);
        } else if self.inputs.is_key_pressed(Keycode::E) {
            self.camera.render_distance += 1;
            println!("Camera zoom set to {}", self.camera.render_distance);
        }
        if self.map.is_none() {
            if self.inputs.is_key_pressed(Keycode::R) {
                self.create_world()?; // Appel de la méthode pour créer la carte
            } else if self.inputs.is_key_pressed(Keycode::L) {
                // self.load_world()?;
            }
        } else if self.inputs.is_key_pressed(Keycode::Space) {
            self.map.as_ref().unwrap().save().expect("Failed to save map");
            println!("Map saved !");
        }

        if let Some(key) = self.inputs.key_pressed.last() {
            if let Ok(dir) = key.to_direction() {
                self.camera.move_dir(dir);
                self.inputs.key_pressed.pop();
            }
        }

        Ok(())
    }

    // Mettre à jour les unités, la carte, les ressources, etc.
    fn update_game_logic(&mut self) {
        let sndr = self.sndr.clone();
        let mut mngr = self.chunk_manager.lock().unwrap();

        // Vérifiez les chunks reçus via le receiver
        while let Ok((key, status)) = self.rcvr.recv_timeout(Duration::new(0, 5_000)) {
            match status {
                Status::Pending => {}
                Status::Ready(_) | Status::Visible(_) => {
                    // Ajoutez ou remplacez le chunk dans la carte une fois prêt
                    if self.map.is_some() {
                        mngr.loaded_chunks.insert(key, status);
                    }
                }
                Status::ToGenerate => {
                    Chunk::generate_async(
                        key,
                        self.map.clone().unwrap().seed,
                        BiomeConfig::default(),
                        sndr.clone()
                    );
                }
                _ => {
                    eprintln!("Statut inconnu pour le chunk {:?}: {:?}", key, status);
                }
            }
        }

        // Vérifiez régulièrement si des chunks en `Pending` doivent être relancés
    }

    // Mettre à jour les animations, états visuels, etc.
    fn update_visuals(&mut self) {
        let mut chunk_manager = self.chunk_manager.lock().unwrap();
        chunk_manager.visible_chunks = Map::visible_chunks(&self.camera);

        for key in chunk_manager.visible_chunks.clone() {
            let status = chunk_manager.loaded_chunks.get(&key.clone());

            let status = match status.clone() {
                Some(status) =>
                    match status.clone().visible() {
                        Ok(status) => status,
                        Err(_) => status.clone(),
                    }
                None => {
                    self.sndr.send((key, Status::ToGenerate)).unwrap();
                    Status::Pending
                }
            };

            chunk_manager.loaded_chunks.insert(key, status);
        }
    }

    fn render(&mut self) {
        if let Some(map) = self.map.clone().as_mut() {
            let mut chunk_manager = self.chunk_manager.lock().unwrap();
            let mut renderer = self.renderer.lock().unwrap();
            // chunk_manager.draw_all(map, &mut renderer, &self.camera);
            chunk_manager.draw(&mut renderer, &self.camera);
        }
    }
}

use std::{
    fs::{ self, DirBuilder },
    io::{ self, Write },
    sync::{ Arc, Mutex },
    time::{ Duration, Instant },
};

use biomes::BiomeConfig;
use chunk_manager::ChunkManager;
use chunk_manager::Draw;
use chunk_manager::Clear;
use chunk_manager::Update;
use coords::Coords;
use sdl2::{ event::Event, keyboard::Keycode, pixels::Color, render::WindowCanvas, Sdl };
use map::{ camera::Camera, renderer::{ Renderer, TILE_SIZE }, Directions, Map };
use unit::{ Unit, MOVING };

use crate::inputs::{ Inputs, ToDirection };
pub const WIN_DEFAULT_WIDTH: u32 = 1000;
pub const WIN_DEFAULT_HEIGHT: u32 = 800;

pub struct Game {
    pub chunk_manager: Arc<Mutex<ChunkManager>>,
    pub renderer: Arc<Mutex<Renderer>>,
    pub last_tick: Instant,
    pub tick_rate: Duration, // Pour contrôler la fréquence des ticks (30 ou 60 fps)

    // pub renderer: Renderer,
    // pub chunk_manager: ChunkManager,

    pub biome_config: BiomeConfig,
    pub camera: Camera,
    pub sdl: Sdl,
    pub events: Vec<Event>,
    pub inputs: Inputs,
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
        let mut chunk_manager = ChunkManager::new();
        let camera = Camera::new(0.0, 0.0);

        Game {
            last_tick: Instant::now(),
            tick_rate: Duration::new(1, 0) / 60, // 60 FPS par défaut
            chunk_manager: Arc::new(Mutex::new(chunk_manager)),
            renderer: Arc::new(Mutex::new(renderer)),

            // renderer,
            // chunk_manager,

            biome_config,
            camera,
            sdl,
            events: Vec::new(),
            inputs: Inputs::new(),
            map: None,
        }
    }

    pub fn load_world(&mut self) -> Result<Map, ()> {
        let path = "./data/a.bin";
        println!("Loading {}", path);
        Ok(Map::load_from_file(path).expect(&format!("Failed to load map at '{}'", path)))
    }

    pub fn create_world(&mut self) -> Result<(), ()> {
        // Vérifier si le dossier "data/" existe, sinon le créer
        if !fs::metadata("./data").is_ok() {
            // Créer le dossier "data/"
            if let Err(e) = DirBuilder::new().recursive(true).create("./data") {
                eprintln!("Erreur lors de la création du dossier 'data/': {}", e);
                return Err(());
            }
        }

        if self.map.is_none() {
            // Demander le nom du fichier de la carte à l'utilisateur
            print!("Entrez le nom du fichier de la carte : ");
            io::stdout().flush().unwrap(); // Assure-toi que le prompt est affiché avant de lire l'entrée.

            let mut file_name = String::new();
            // Créer le chemin du fichier
            file_name = if cfg!(test) == false {
                "test".to_string()
            } else {
                let mut file_name = String::new();
                io::stdin().read_line(&mut file_name).unwrap();
                file_name.trim().to_string() // Supprimer les espaces inutiles
            };

            let path = format!("./data/{}.bin", file_name);

            // Charger la carte avec le fichier donné

            self.map = Some(Map::new(&path));
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
            println!("Camera speed set to{}", self.camera.speed);
        } else if self.inputs.is_key_pressed(Keycode::KP_PLUS) {
            self.camera.speed += 0.01;
            println!("Camera speed set to{}", self.camera.speed);
        } else if self.inputs.is_key_pressed(Keycode::A) && self.camera.render_distance > 1 {
            self.camera.render_distance -= 1;
            println!("Camera zoom set to{}", self.camera.render_distance);
        } else if self.inputs.is_key_pressed(Keycode::E) {
            self.camera.render_distance += 1;
            println!("Camera zoom set to{}", self.camera.render_distance);
        }

        if self.map.is_none() {
            if self.inputs.is_key_pressed(Keycode::R) {
                self.create_world()?; // Appel de la méthode pour créer la carte
            } else if self.inputs.is_key_pressed(Keycode::L) {
                self.load_world()?;
            }
        }else if self.inputs.is_key_pressed(Keycode::Space){
            self.map.as_ref().unwrap().save();
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
        // if self.map.is_some() {
        //     self.chunk_manager
        //         .lock()
        //         .expect("...?")
        //         .update(self.map.clone().unwrap(), &self.camera);
        // }
    }

    // Mettre à jour les animations, états visuels, etc.
    fn update_visuals(&mut self) {}

    // Dessiner la scène avec le renderer
    fn render(&mut self) {
        if self.map.is_none() {
            // No map, no render
            // Makes it safe to use self.map.unwrap() in this method
            return;
        }

        // Accéder au chunk_manager
        let mut chunk_manager = match self.chunk_manager.lock() {
            Ok(lock) => lock,
            Err(_) => {
                eprintln!("Impossible de verrouiller chunk_manager");
                return;
            }
        };

        chunk_manager.update(&mut self.map.clone().unwrap(), &self.camera);

        // Essayez d'obtenir une référence mutable au renderer
        if let Ok(mut renderer) = self.renderer.lock() {
            // let (offset_x, offset_y) = self.camera.get_offset(window_width, window_height);

            chunk_manager.draw(&mut renderer, &self.camera);
        } else {
            // Gérer le cas où le verrouillage du renderer échoue
            eprintln!("Impossible de verrouiller le renderer");
        }
    }
}

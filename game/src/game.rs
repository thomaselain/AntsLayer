use std::sync::mpsc::Sender;

use biomes::Config;
use chunk::{ thread::Status, Chunk };
#[allow(unused_imports)]
use chunk_manager::Draw;
#[allow(unused_imports)]
use chunk_manager::DrawAll;

use coords::aliases::TilePos;
use coords::Coords;
use sdl2::{ keyboard::Keycode, pixels::Color };
use map::{ Map, WORLD_STARTING_AREA };

#[allow(unused_imports)]
use unit::{ Unit, MOVING };

use crate::Game;

pub const WIN_DEFAULT_WIDTH: u32 = 1000;
pub const WIN_DEFAULT_HEIGHT: u32 = 800;
impl Game {
    // pub fn load_world(&mut self) -> Result<Map, ()> {
    //     let path = "./data/test_world";
    //     println!("Loading {}", path);
    //     // Ok(Map::load(path).expect(&format!("Failed to load map at '{}'", path)))
    // }

    pub fn create_world(&mut self) -> Result<(), String> {
        self.map = Some(Map::init_test());

        let half_size = WORLD_STARTING_AREA / 2;

        for x in -half_size..=half_size {
            for y in -half_size..=half_size {
                let key = Coords::new(x, y);
                let (_height, biome) = self.config
                    .clone()
                    .biome_from_coord((key.x(), key.y()), 0);
                Chunk::generate_from_biome(key, self.map.clone().unwrap().seed, biome);
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

    fn render(&mut self) {
        if self.map.is_some() {
            let chunk_manager = self.chunk_manager.lock().unwrap();
            let mut renderer = self.renderer.lock().unwrap();
            // chunk_manager.draw_all(&mut self.map.clone().unwrap(), &mut renderer, &self.camera);
            chunk_manager.draw(&mut renderer, &self.camera);
        }
    }

    pub fn run(&mut self) {
        // Boucle de jeu
        'running: loop {
            let mut event_pump = self.sdl.event_pump().unwrap();

            if let Ok(mut renderer) = self.renderer.lock() {
                renderer.canvas.clear();
                renderer.canvas.set_draw_color(Color::RGB(0, 0, 0));
            } else {
                panic!();
            }

            if self.inputs.is_key_pressed(Keycode::Escape) {
                break 'running;
            }

            for event in event_pump.poll_iter() {
                self.events.push(event);
            }
            self.tick(); // Dessiner la carte et les unités
            if let Ok(mut renderer) = self.renderer.lock() {
                renderer.canvas.present();
            } else {
                panic!();
            }
        }
    }
}

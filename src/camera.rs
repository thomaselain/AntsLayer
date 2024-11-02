use crate::coords::Coords;
use crate::terrain;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Coords,
    pub zoom: f32,
    pub screen_width: u32,
    pub screen_height: u32,
    pub tile_size: u32, // Taille de tuile de base
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Camera {
            position: Coords { x: 0, y: 0 },
            zoom: 1.0,
            screen_width,
            screen_height,
            tile_size: terrain::TILE_SIZE, // Définir une taille de tuile de base
        }
    }

    pub fn world_to_screen(&self, world_coords: Coords) -> Coords {
        Coords {
            x: ((world_coords.x as f32 - self.position.x as f32) * self.zoom) as i32,
            y: ((world_coords.y as f32 - self.position.y as f32) * self.zoom) as i32,
        }
    }

    pub fn screen_to_world(&self, screen_coords: Coords) -> Coords {
        Coords {
            x: (screen_coords.x as f32 / self.zoom) as i32 + self.position.x,
            y: (screen_coords.y as f32 / self.zoom) as i32 + self.position.y,
        }
    }

    pub fn is_on_screen(&self, world_coords: Coords) -> bool {
        let tile_size = (self.zoom * self.tile_size as f32) as i32; // Taille de tuile basée sur le zoom
        let screen_coords = self.world_to_screen(world_coords);
        screen_coords.x >= 0
            && screen_coords.x < self.screen_width as i32 + tile_size
            && screen_coords.y >= 0
            && screen_coords.y < self.screen_height as i32 + tile_size
    }

    pub fn apply_zoom(&mut self, mouse_x: i32, mouse_y: i32) {
        let previous_zoom = self.zoom; // Garder le zoom précédent pour ajuster la position
        self.zoom *= self.zoom; // Appliquer le zoom

        // Mettre à jour la taille des tuiles
        self.tile_size = (terrain::TILE_SIZE as f32 * self.zoom) as u32;


        // Obtenir les coordonnées du monde avant le zoom
        let world_coords_before_zoom = self.screen_to_world(Coords {
            x: mouse_x,
            y: mouse_y,
        });
        // Ajuster la position de la caméra en fonction du changement de zoom
        self.position.x += ((world_coords_before_zoom.x
            - self
                .screen_to_world(Coords {
                    x: (mouse_x as f32 * previous_zoom) as i32,
                    y: (mouse_y as f32 * previous_zoom) as i32,
                })
                .x) as f32
            * (1.0 - self.zoom)) as i32;

        self.position.y += ((world_coords_before_zoom.y
            - self
                .screen_to_world(Coords {
                    x: (mouse_x as f32 * previous_zoom) as i32,
                    y: (mouse_y as f32 * previous_zoom) as i32,
                })
                .y) as f32
            * (1.0 - self.zoom)) as i32;
    }
}

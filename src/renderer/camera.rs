use std::{ collections::HashMap };

use crate::{ ant::Direction, chunk::{ manager::LoadedChunk, CHUNK_HEIGHT, CHUNK_WIDTH } };

use super::Renderer;

// Camera
impl<'ttf> Renderer<'ttf> {
    pub fn camera_range_i32(&self) -> (i32, i32, i32, i32) {
        // Render distance calucation
        (
            (-self.camera.0 - self.view_distance) / (CHUNK_WIDTH as i32),
            (-self.camera.0 + self.view_distance) / (CHUNK_WIDTH as i32),
            (-self.camera.1 - self.view_distance) / (CHUNK_WIDTH as i32),
            (-self.camera.1 + self.view_distance) / (CHUNK_WIDTH as i32),
        )
    }

    /// Filtre la liste des LoadedChunk pour ne garder que ceux visibles
    pub fn filter_visible_chunks(
        &self,
        chunks: &mut HashMap<(i32, i32), LoadedChunk>
    )  {
        let (x_min, x_max, y_min, y_max) = self.camera_range_i32();
        for x in x_min..x_max {
            for y in y_min..y_max {
                if !chunks.contains_key(&(x, y)){
                    chunks.remove(&(x,y));
                }
            }
        }
    }
    pub fn increase_view_dist(&mut self) -> Result<(), ()> {
        self.view_distance += 1;
        Ok(())
    }
    pub fn decrease_view_dist(&mut self) -> Result<(), ()> {
        if self.view_distance > 0 {
            self.view_distance -= 1;
        } else {
            self.view_distance = 0;
        }
        Ok(())
    }
    pub fn zoom_in(&mut self) -> Result<(), ()> {
        self.tile_size += 1;
        Ok(())
    }
    pub fn zoom_out(&mut self) -> Result<(), ()> {
        self.tile_size -= 1;
        Ok(())
    }

    pub fn move_camera(&mut self, dir: Direction) {
        let (x, y, z) = self.camera;
        let speed = self.camera_speed as i32;

        let mv = match dir {
            Direction::Up if z < (CHUNK_HEIGHT as i32) => (0, 0, 1),
            Direction::Down if z > 0 => (0, 0, -1),

            Direction::North => (0, speed, 0),
            Direction::East => (-speed, 0, 0),
            Direction::South => (0, -speed, 0),
            Direction::West => (speed, 0, 0),

            // Don't move if nothing matches
            _ => { (0, 0, 0) }
        };

        self.camera = (x + mv.0, y + mv.1, z + mv.2);
    }
}

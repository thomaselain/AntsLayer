use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::{ fmt::Debug };

use crate::{
    chunk::generation::{ MapShape, STARTING_AREA, STARTING_MAP_SHAPE },
    renderer::{ CLOUDS_HEIGHT, FONT_SIZE },
    Game,
};

impl Debug for MapShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SQUARE => write!(f, "SQUARE"),
            Self::RECT => write!(f, "RECT"),
            Self::ROUND => write!(f, "ROUND"),
        }
    }
}

const DEBUG_FONT_SIZE: u32 = FONT_SIZE as u32;
const DEBUG_BACKGROUND_COLOR: Color = Color::RGBA(5, 5, 5, 150);

// impl<'ttf> Game<'ttf> {
//     pub fn display_info(&mut self) -> Result<(), String> {
//         if
//             let (Some(c_mngr), Some(a_mngr)) = (
//                 self.chunk_manager.lock().ok(),
//                 self.ant_manager.lock().ok(),
//             )
//         {
//             // Black background
//             self.renderer.canvas.set_draw_color(DEBUG_BACKGROUND_COLOR);
//             self.renderer.canvas.fill_rect(Rect::new(0, 0, 250, DEBUG_FONT_SIZE * 11))?;

//             self.display_info_at(
//                 format!(
//                     "win:{:?}|||FPS  :{:?}|||TPS  :{:?}|||",
//                     // FPS CALCULATION
//                     self.renderer.dims,
//                     self.fps,
//                     self.tps
//                 ),
//                 0
//             )?;
//             self.display_info_at(format!("Camera  : {:?}", self.renderer.camera), 1)?;
//             self.display_info_at(format!("Time       : {:.1?}s", self.elapsed_secs()), 2)?;
//             self.display_info_at(format!("Tile size  : {:?}", self.renderer.tile_size), 3)?;
//             self.display_info_at(format!("Loaded chunks : {:?}", c_mngr.loaded_chunks.len()), 4)?;
//             self.display_info_at(
//                 format!(
//                     "Visible chunks : {:?}",
//                     self.renderer.visible_chunks(c_mngr.loaded_chunks.clone()).len()
//                 ),
//                 5
//             )?;
//             self.display_info_at(format!("Clouds height : {:?}", CLOUDS_HEIGHT), 6)?;

//             if let Some(joette) = a_mngr.ants.first() {
//                 self.display_info_at(format!("Joette's pos {:?}", joette.pos), 9)?;
//             }

//             self.display_info_at(
//                 format!("map is {:?} of size {:?}", STARTING_MAP_SHAPE, STARTING_AREA),
//                 10
//             )?;
//         }
//         Ok(())
//     }
//     pub fn display_info_at(&self, info: String, index: i32) -> Result<(), String> {
//         // Load font
//         // Text to display
//         let text = info;

//         // Turn text into a surface and then a texture
//         let surface = self.renderer.font
//             .render(&text)
//             .blended(Color::WHITE)
//             .map_err(|e| e.to_string())?;

//         let texture_creator = self.renderer.canvas.texture_creator();
//         let texture = texture_creator
//             .create_texture_from_surface(&surface)
//             .map_err(|e| e.to_string())?;

//         // write it
//         let target = Rect::new(
//             10,
//             (DEBUG_FONT_SIZE as i32) * index,
//             surface.width(),
//             surface.height()
//         );
//         // self.renderer.canvas.copy(&texture, None, Some(target))?;
//         Ok(())
//     }
// }

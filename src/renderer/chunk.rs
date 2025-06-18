use sdl2::pixels::Color;

use crate::{
    ant::Ant,
    chunk::{
        index::{ self, flatten_index_i32 },
        manager::LoadedChunk,
        tile::TileFlag,
        ChunkContent,
    },
};

#[allow(unused)]
use super::{
    Renderer,
    CLOUDS_HEIGHT,
    CLOUDS_RENDERING,
    GRID_COLOR,
    IS_GRID_ENABLED,
    MAX_RENDERING_DEPTH,
};

/// Chunk rendering
///
impl LoadedChunk {
    #[allow(unused)]
    fn biome_render(&self, renderer: &mut Renderer, draw_pos: (i32, i32), timestamp: f64) {
        // renderer.draw_chunk(draw_pos, self.biome_id.into());

        // if cfg!(test) {
        //     let (d, h, m) = crate::time::game_time(timestamp);
        //     let c = Color::RGBA(255, 0, 255, 10 + 10 * (h as u8));
        //     renderer.draw_chunk(draw_pos, c);
        // }
    }
    pub fn render(&self, renderer: &mut Renderer, ants: &Vec<Ant>, timestamp: f64) {
        if !cfg!(test) && renderer.tile_size < 5 {
            let (world_x, world_y) = Renderer::to_world_coords((self.pos.0, self.pos.1), (0, 0));
            let draw_pos = renderer.tile_to_screen_coords((world_x, world_y));
            self.biome_render(renderer, draw_pos, timestamp);
            return;
        }

        let mut tiles_to_draw = Vec::with_capacity((MAX_RENDERING_DEPTH as usize) + 1);

        for index in 0..ChunkContent::len() {
            let (x, y, z) = index::to_xyz(index);

            if z == renderer.camera.2 {
                let (world_x, world_y) = Renderer::to_world_coords(
                    (self.pos.0, self.pos.1),
                    (x, y)
                );
                let draw_pos = renderer.tile_to_screen_coords((world_x, world_y));

                ////////////////////////////////////////////////////////////////
                //////////////////     FOG     RENDERING  //////////////////////
                ////////////////////////////////////////////////////////////////
                let mut depth = 1;
                let mut current_z = z;
                'find_deepest: loop {
                    let idx = flatten_index_i32((x, y, current_z));
                    let tile = self.access_tile_from_index(&idx);

                    tiles_to_draw.push(tile);

                    if
                        // current tile is not transparent
                        !tile.properties.contains(TileFlag::TRANSPARENT) ||
                        // Reached bottom
                        current_z == 0 ||
                        // Dont draw too much
                        depth >= MAX_RENDERING_DEPTH
                    {
                        break 'find_deepest;
                    }

                    current_z -= 1;
                    depth += 1;
                }
                ////////////////////////////////////////////////////////////////

                // Draw deepest tile found first
                if let Some(bottom_tile) = tiles_to_draw.pop() {
                    let c = bottom_tile.color();
                    renderer.fill_rect(draw_pos, c);
                }
                // Grey filter for walls
                if tiles_to_draw.is_empty() {
                    let c = Color::RGBA(25, 25, 25, 175);
                    renderer.fill_rect(draw_pos, c);
                }

                // Draw transparent blocks
                'bottom_to_top: loop {
                    if let Some(tile) = tiles_to_draw.pop() {
                        ////////////////////////////////////////////////////////////////
                        ////////////////////  Ants  Rendering //////////////////////////
                        ////////////////////////////////////////////////////////////////
                        for a in ants {
                            if a.pos.2 == z {
                                a.render(renderer);
                                continue;
                            }
                        }
                        ////////////////////////////////////////////////////////////////

                        let mut fog = tile.color();
                        fog.a += tile.color().a;
                        renderer.fill_rect(draw_pos, fog);
                    } else {
                        break 'bottom_to_top;
                    }
                }

                ////////////////////////Clouds//////////////////////////////////
                if CLOUDS_RENDERING && z >= CLOUDS_HEIGHT {
                    renderer.draw_cloud(self.pos, (x, y), draw_pos, timestamp);
                    continue;
                }

                //////////////////////// Grid //////////////////////////////////
                if renderer.is_grid_enabled {
                    renderer.rect(draw_pos, GRID_COLOR);
                }

                tiles_to_draw.clear();
            }
        }
    }
}

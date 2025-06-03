use crate::{ant::Ant, chunk::{index::flatten_index_i32, tile::TileFlag, Chunk, ChunkContent}};

use super::{Renderer, CLOUDS_HEIGHT, CLOUDS_RENDERING, CLOUD_COLOR, GRID_COLOR, IS_GRID_ENABLED, MAX_RENDERING_DEPTH};

/// Chunk rendering
///
impl Chunk {
    pub fn render(
        &self,
        renderer: &mut Renderer,
        // Chunk coordinates
        (pos_x, pos_y): (i32, i32),
        ants: &Vec<Ant>,
        timestamp: f64
    ) {
        let mut tiles_to_draw = Vec::with_capacity((MAX_RENDERING_DEPTH as usize) + 1);

        for index in 0..ChunkContent::len() {
            let (x, y, z) = ChunkContent::index_to_xyz(index);

            if z == renderer.camera.2 {
                let (world_x, world_y) = Renderer::to_world_coords((pos_x, pos_y), (x, y));
                let draw_pos = renderer.tile_to_screen_coords((world_x, world_y));

                ////////////////////////////////////////////////////////////////
                //////////////////     FOG     RENDERING  //////////////////////
                ////////////////////////////////////////////////////////////////
                let mut depth = 1;
                let mut current_z = z;
                'find_deepest: loop {
                    let idx = flatten_index_i32((x, y, current_z));
                    let tile = &self.content[idx];

                    tiles_to_draw.push(tile);

                    if
                        // current tile is not transparent
                        !tile.properties.contains(TileFlag::TRANSPARENT) ||
                        // tile.tile_type != TileType::Gas(Gas::Air) ||
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

                // Draw the fog layer
                // And water depth
                'bottom_to_top: loop {
                    if let Some(tile) = tiles_to_draw.pop() {
                        ////////////////////////////////////////////////////////////////
                        ////////////////////  Ants  Rendering //////////////////////////
                        ////////////////////////////////////////////////////////////////
                        if ants.len() > 0 {
                            // todo!("Chunk at {:?} has {:?} ants", (pos_x, pos_y), ants.len());
                            for a in ants {
                                if a.pos.2 <= z {
                                    a.render(renderer);
                                }
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

                ////////////////////////////////////////////////////////////////
                ////////////////////////Clouds//////////////////////////////////
                ////////////////////////////////////////////////////////////////
                if CLOUDS_RENDERING && z >= CLOUDS_HEIGHT {
                    let mut cloud = CLOUD_COLOR;
                    // Convert into world coords f64
                    // Allows use of perlin.get[coords]
                    let (x, y) = Renderer::to_world_coords((pos_x, pos_y), (x, y));
                    let (x, y, z) = (x as f64, y as f64, CLOUDS_HEIGHT as f64);

                    // Find cloud value
                    let cloud_value = ((cloud.a as f64) +
                        renderer.noise.get_cloud_value(
                            x + timestamp * 1.5,
                            y + timestamp * 1.1,
                            z,
                            timestamp / 69.0
                        ) *
                            255.0) as u8;
                    cloud.a = match cloud_value {
                        0..50 => 150,
                        50..75 => 100,
                        75..79 => 50,
                        140..150 => 75,
                        150..160 => 15,
                        170..180 => 175,
                        _ => 0,
                    };
                    renderer.fill_rect(draw_pos, cloud);
                }

                ////////////////////////////////////////////////////////////////
                if IS_GRID_ENABLED {
                    renderer.rect(draw_pos, GRID_COLOR);
                }
                ////////////////////////////////////////////////////////////////

                tiles_to_draw.clear();
            }
        }
    }
}

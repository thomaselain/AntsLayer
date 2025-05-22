use std::ops::{ Index, IndexMut };

use super::{ tile::Tile, ChunkContent, CHUNK_WIDTH };

pub fn flatten_index_i32((x, y, z): (i32, i32, i32)) -> usize {
    let (x, y, z) = (x as usize, y as usize, z as usize);
    x + y * CHUNK_WIDTH + z * CHUNK_WIDTH * CHUNK_WIDTH
}
pub fn flatten_index_usize((x, y, z): (usize, usize, usize)) -> usize {
    x + y * CHUNK_WIDTH + z * CHUNK_WIDTH * CHUNK_WIDTH
}

/// ChunkContent[(i32, i32, i32)]
impl IndexMut<(i32, i32, i32)> for ChunkContent {
    fn index_mut(&mut self, index: (i32, i32, i32)) -> &mut Self::Output {
        &mut self.0[flatten_index_i32(index)]
    }
}
impl Index<(i32, i32, i32)> for ChunkContent {
    type Output = Tile;
    fn index(&self, index: (i32, i32, i32)) -> &Self::Output {
        &self.0[flatten_index_i32(index)]
    }
}

/// ChunkContent[usize]
impl Index<usize> for ChunkContent {
    type Output = Tile;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for ChunkContent {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// ChunkContent[(usize, usize, usize)]
impl Index<(usize, usize, usize)> for ChunkContent {
    type Output = Tile;
    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        &self.0[flatten_index_usize(index)]
    }
}

impl IndexMut<(usize, usize, usize)> for ChunkContent {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        &mut self.0[flatten_index_usize(index)]
    }
}

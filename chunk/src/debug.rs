use std::fmt;

use tile::{ FluidType, TileType };

use crate::{ Chunk, CHUNK_HEIGHT, CHUNK_WIDTH };

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk ({}x{}):", CHUNK_WIDTH, CHUNK_WIDTH)?;

        for z in 0..CHUNK_HEIGHT {
            for y in 0..CHUNK_WIDTH {
                for x in 0..CHUNK_WIDTH {
                    let tile = &self.layers[z].tiles[y][x];
                    let symbol = match tile.tile_type {
                        TileType::Empty => ' ',
                        TileType::Wall => '#',
                        TileType::Floor => '_',
                        TileType::Rock => '&',
                        TileType::Grass => ',',
                        TileType::Sand => 'X',
                        TileType::Dirt => '.',
                        TileType::Fluid(liquid) =>
                            match liquid {
                                FluidType::Magma => 'M',
                                FluidType::Water => 'w',
                                FluidType::Deep_water => 'W',
                            }
                        TileType::Custom(_) => '?',
                    };
                    write!(f, "{} ", symbol)?;
                }
                writeln!(f)?; // Nouvelle ligne après chaque rangée
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tile::TileType;

    use crate::Chunk;

    #[test]
    fn chunk_debug_printing() {
        // Crée un Chunk pour vérifier l'implémentation Debug
        let chunk = Chunk::default();

        // Vérifie que l'implémentation Debug fonctionne en affichant dans le terminal
        println!("{:?}", chunk); // Cela devrait afficher les informations de Chunk

        // On peut aussi vérifier des éléments de base comme un tile spécifique
        assert_eq!(chunk.layers[0].tiles[0][0].tile_type, TileType::Empty); // Vérifie que la première case est bien vide
    }
}

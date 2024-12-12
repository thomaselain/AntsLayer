use std::fmt;

use tile::TileType;

use crate::{ Chunk, CHUNK_SIZE };

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk ({}x{}):", CHUNK_SIZE, CHUNK_SIZE)?;

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let tile = &self.tiles[y][x];
                let symbol = match tile.tile_type {
                    TileType::Empty => '.', // Espace pour les tuiles vides
                    TileType::Wall => '#', // Mur
                    TileType::Floor => '_', // Sol
                    TileType::Liquid => '~', // Liquide
                    TileType::Custom(_) => '?', // Type personnalisé
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?; // Nouvelle ligne après chaque rangée
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tile::TileType;

    use crate::Chunk;

    #[test]
    fn test_chunk_debug_printing() {
        // Crée un Chunk pour vérifier l'implémentation Debug
        let chunk = Chunk::new();

        // Vérifie que l'implémentation Debug fonctionne en affichant dans le terminal
        println!("{:?}", chunk); // Cela devrait afficher les informations de Chunk

        // On peut aussi vérifier des éléments de base comme un tile spécifique
        assert_eq!(chunk.tiles[0][0].tile_type, TileType::Empty); // Vérifie que la première case est bien vide
    }
}
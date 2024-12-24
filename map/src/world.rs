use std::collections::HashMap;

use chunk::ChunkPath;
use rand::Rng;

use crate::{ Map, WORLDS_FOLDER };

impl Map {
    pub fn init_world_folder(name: &str) -> Result<(), String> {
        // Vérifier si le dossier "data/worlds" existe, sinon le créer
        if !std::fs::metadata(format!("{}/{}", WORLDS_FOLDER, name)).is_ok() {
            // Créer le dossier "data/"
            if
                let Err(e) = std::fs::DirBuilder
                    ::new()
                    .recursive(true)
                    .create(format!("{}/{}", WORLDS_FOLDER, name))
            {
                eprintln!("Erreur lors de la création du dossier '{}': {}", WORLDS_FOLDER, e);
                return Err(format!("{}/{}", WORLDS_FOLDER, name));
            }
        }
        Ok(())
    }

    pub fn init_world(name: &str) -> Result<Self, String> {
        let mut rng = rand::thread_rng();
        let seed = rng.gen_range(0..10);

        Ok(Self {
            seed,
            path: format!("{}/{}", WORLDS_FOLDER, name),
            chunks: HashMap::new(),
        })
    }

    // Sauvegarder la map entière
    pub fn save(self) -> std::io::Result<()> {
        for (key, chunk) in self.chunks {
            chunk.save(ChunkPath::new(&self.path, key))?;
        }
        Ok(())
    }

    // Charger la map entière
    // pub fn load(path: &str) -> Result<Map, io::Error> {
    //     let file = File::open(path).ok();
    //     let map = bincode::deserialize_from(file.expect("failed to open map file"));
    //     Ok(map.ok().expect("Error while loading map?"))
    // }
}

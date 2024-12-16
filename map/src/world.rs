use std::collections::HashMap;

use biomes::BiomeConfig;
use chunk::{Chunk, ChunkPath};
use rand::Rng;

use crate::{Map, WORLDS_FOLDER, WORLD_STARTING_AREA};

impl Map{
fn init_world(name: &str) -> Result<Self, String> {
    let mut rng = rand::thread_rng();
    let seed = rng.gen_range(0..10);

    // Vérifier si le dossier "data/worlds" existe, sinon le créer
    if !std::fs::metadata(format!("{}{}", WORLDS_FOLDER, name).to_string()).is_ok() {
        // Créer le dossier "data/"
        if
            let Err(e) = std::fs::DirBuilder
                ::new()
                .recursive(true)
                .create(format!("{}{}", WORLDS_FOLDER, name).to_string())
        {
            eprintln!("Erreur lors de la création du dossier '{}': {}", WORLDS_FOLDER, e);
            return Err(format!("{}{}", WORLDS_FOLDER, name).to_string());
        }
    }

    Ok(Self {
        seed,
        path: format!("{}{}", WORLDS_FOLDER, name).to_string(),
        chunks: HashMap::new(),
    })
}

pub fn create_world(name: &str) -> Result<Self, String> {
    let mut map = Self::init_world(name)?;

    let half_size = WORLD_STARTING_AREA / 2;

    for x in -half_size..=half_size {
        for y in -half_size..=half_size {
            let ((x,y),chunk) = Chunk::generate_from_biome(x,y,map.seed, BiomeConfig::default());
            map.add_chunk((x, y), chunk).expect("Failed to add chunk");
        }
    }

    Ok(map)
}

pub fn new(name: &str) -> Result<Self, String> {
    Self::create_world(name)
}

    // Sauvegarder la map entière
    pub fn save(&self) -> std::io::Result<()> {
        for ((x, y), chunk) in &self.chunks {
            chunk.save(ChunkPath::build(self.path.clone(), *x, *y).expect("Failed to save chunk"))?;
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
use coords::Coords;
use rand::Rng;

use crate::DEFAULT_BIOME_NAME;

use super::*;

#[test]
fn biomes_loading() {
    let cfg = Config::new();
    for biome in &cfg.biomes {
        assert!(!biome.name.is_empty());
        assert!(!biome.thresholds.is_empty());
        assert!(!biome.noise_layers.is_empty());
        // println!("{:?}", biome);
    }
    eprintln!("{:#?}", cfg);
}

#[test]
fn combined_noise() {
    const TEST_RANGE: std::ops::Range<i32> = 0..100;
    let cfg = Config::new().get_biome(DEFAULT_BIOME_NAME);

    for x in TEST_RANGE {
        let seed: u32 = rand::thread_rng().gen_range(TEST_RANGE) as u32;
        let perlin = Perlin::new(seed);
        let value = cfg.clone().combined_noise(&perlin, Coords::new(x,1,1));
        let tile_type = cfg.clone().tile_type_from_noise(value);
        eprintln!("{:.2}  -->  {:?}", value, tile_type);
        assert!((-1.0..=1.0).contains(&value), "Noise value out of range");
    }
}

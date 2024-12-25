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
    println!("{:#?}", cfg.get_biome(DEFAULT_BIOME_NAME));
}

#[test]
fn combined_noise() {
    const TEST_RANGE: std::ops::Range<i32> = 0..15;
    let cfg = Config::new().get_biome(DEFAULT_BIOME_NAME);

    for _ in TEST_RANGE {
        let seed: u32 = rand::thread_rng().gen_range(TEST_RANGE) as u32;
        let perlin = BiomeConfig::noise_from_seed(seed);
        let value = cfg.clone().combined_noise(&perlin, 0.5, 0.5);
        let tile_type =cfg.clone().tile_type_from_noise(value);
        eprintln!("{:.2}  -->  {:?}", value, tile_type);
        assert!((-1.0..=1.0).contains(&value), "Noise value out of range");
    }
}

use super::*;

#[test]
fn biomes_loading() {
    let cfg = Config::new();
    for biome in cfg.biomes {
        println!("{:?}", biome);
    }
}

#[test]
fn combined_noise() {
    let cfg = Config::new();

    let perlin = BiomeConfig::noise_from_seed(0);
    let value = cfg.default_biome().combined_noise(&perlin, 0.5, 0.5);

    assert!((-1.0..=1.0).contains(&value), "Noise value out of range");
}
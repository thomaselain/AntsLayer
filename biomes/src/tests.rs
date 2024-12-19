use super::*;

#[test]
fn biomes_loading() {
    let config = Config::new();
    for biome in config.biomes {
        println!("{:?}", biome);
    }
}

#[test]
fn combined_noise() {
    let biome = BiomeConfig::default();

    let perlin = BiomeConfig::noise_from_seed(0);
    let value = biome.combined_noise(&perlin, 0.5, 0.5);

    assert!(value >= -1.0 && value <= 1.0, "Noise value out of range");
}
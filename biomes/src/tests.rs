use super::*;

#[test]
pub fn test_biomes_loading() {
    let config = Config::load().expect("Failed to load biomes_config file");
    for biome in config.biomes {
        println!("{:?}", biome);
    }
}

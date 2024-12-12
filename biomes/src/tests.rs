use super::*;

#[test]
pub fn biomes_loading() {
    let config = Config::load().expect("Failed to load biomes_config file");
    for biome in config.biomes {
        println!("{:?}", biome);
    }
}

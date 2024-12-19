use super::*;

#[test]
pub fn biomes_loading() {
    let config = Config::new();
    for biome in config.biomes {
        println!("{:?}", biome);
    }
}
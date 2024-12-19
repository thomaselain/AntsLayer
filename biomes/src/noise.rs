use serde::Deserialize;

use crate::BiomeConfig;

#[derive(Deserialize, Debug, Clone)]
pub struct NoiseLayer {
    pub scale: f64,
    pub weight: f64,
}

impl BiomeConfig {
    pub fn combined_noise(&self, perlin: &noise::Perlin, nx: f64, ny: f64) -> f64 {
        let mut value = 0.0;
        let mut total_weight = 0.0;

        for layer in &self.noise_layers {
            let layer_value = noise::NoiseFn::get(perlin, [nx * layer.scale, ny * layer.scale]);
            value += layer_value * layer.weight;
            total_weight += layer.weight;
        }

        if total_weight > 0.0 {
            value /= total_weight; // Normalisation
        }

        value
    }
}

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct NoiseLayer {
    pub scale: f64,
    pub weight: f64,
}
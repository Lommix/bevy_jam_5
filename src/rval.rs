use rand::random;

#[allow(unused)]
pub mod prelude {
    pub use super::{RVal, Random};
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RVal<T>(T, f32);

pub trait Random<T> {
    fn random(&self) -> T;
}

impl Random<u32> for RVal<u32> {
    fn random(&self) -> u32 {
        let r = (random::<f32>() - 0.5) * 2.;
        self.0 + (self.0 as f32 * r * self.1) as u32
    }
}

impl Random<i32> for RVal<i32> {
    fn random(&self) -> i32 {
        let r = (random::<f32>() - 0.5) * 2.;
        self.0 + (self.0 as f32 * r * self.1) as i32
    }
}

impl Random<f32> for RVal<f32> {
    fn random(&self) -> f32 {
        let r = (random::<f32>() - 0.5) * 2.;
        self.0 + self.0 * r * self.1
    }
}

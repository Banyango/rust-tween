
pub trait Lerp {
    fn lerp(&self, b:&Self, f: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(&self, b:&f32, f: f32) -> f32 {
        (self * ((1.0 as f32) - f)) + (b * f)
    }
}

impl Lerp for f64 {
    fn lerp(&self, b:&f64, f: f32) -> f64 {
        (self * ((1.0 as f64) - (f as f64))) + (b * (f as f64))
    }
}

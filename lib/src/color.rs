use crate::Vec3;

pub type Color = Vec3;

pub const WHITE: Color = Vec3::new(1.0, 1.0, 1.0);
pub const SKY_BLUE: Color = Vec3::new(0.5, 0.7, 1.0);

pub fn mix(a: Color, b: Color, t: f64) -> Color {
    (1.0 - t) * a + t * b
}

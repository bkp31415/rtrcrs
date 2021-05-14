use std::fmt;

/// Defines a pixel's color as an RGB value.
pub type Color = crate::Vec3;

impl fmt::Display for Color {
    /// Write the translated \[0,255\] value of each Color.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (256.0 * self.x()) as u8,
            (256.0 * self.y()) as u8,
            (256.0 * self.z()) as u8,
        )
    }
}

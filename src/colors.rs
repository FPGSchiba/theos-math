use std::io::Write;

use crate::vec3::Vec3;

// Type alias
pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    // Write the translated [0, 255] value of each color component
    let r = (255.999 * pixel_color.x()) as i32;
    let g = (255.999 * pixel_color.y()) as i32;
    let b = (255.999 * pixel_color.z()) as i32;
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_color() {
        let mut out = Vec::new();
        write_color(&mut out, Color::new(0.2, 0.3, 0.4));
        assert_eq!(out, b"51 76 102\n".to_vec());
    }
}
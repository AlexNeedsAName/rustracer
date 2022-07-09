use std::fs::File;
use std::io::BufWriter;
use std::ops::{Mul, Add};
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[allow(dead_code)]
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        return Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        };
    }
    pub fn to_hex(&self) -> String {
        return format!("#{:02x}{:02x}{:02x}", (self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8);
    }

    pub fn overlay(&self, other: Color) -> Color {
        return Color {
            r: self.r * self.a + other.r * (1.0-self.a),
            g: self.g * self.a + other.g * (1.0-self.a),
            b: self.b * self.a + other.b * (1.0-self.a),
            a: other.a
        }
    }

    pub fn average(&self, other: Color, alpha: f32) -> Color {
        return Color {
            r: self.r * alpha + other.r * (1.0 - alpha),
            g: self.g * alpha + other.g * (1.0-alpha),
            b: self.b * alpha + other.b * (1.0-alpha),
            a: self.a * alpha + other.a * (1.0-alpha),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        return Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        };
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        return Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        };
    }
}

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

#[allow(dead_code)]
impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let channels = 4; // RGBA
        return Image {
            width: width,
            height: height,
            pixels: vec![0; width * height * channels],
        };
    }

    pub fn get_width(&self) -> u32 {
        return u32::try_from(self.width).unwrap();
    }

    pub fn get_height(&self) -> u32 {
        return u32::try_from(self.height).unwrap();
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let base = x * 4 + y * self.width * 4;
        return Color {
            r: self.pixels[base] as f32 / 255.0,
            g: self.pixels[base + 1] as f32 / 255.0,
            b: self.pixels[base + 2] as f32 / 255.0,
            a: self.pixels[base + 3] as f32 / 255.0,
        };
    }

    pub fn set_pixelu32(&mut self, x: u32, y: u32, color: Color) {
        return self.set_pixel(
            usize::try_from(x).unwrap(),
            usize::try_from(y).unwrap(),
            color,
        );
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let base = x * 4 + y * self.width * 4;
        self.pixels[base + 0] = (color.r * 255.0) as u8;
        self.pixels[base + 1] = (color.g * 255.0) as u8;
        self.pixels[base + 2] = (color.b * 255.0) as u8;
        self.pixels[base + 3] = (color.a * 255.0) as u8;
        //        return self;
    }

    pub fn save(&self, filename: &String) {
        let path = Path::new(filename);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(
            w,
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
        );
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&self.pixels).unwrap(); // Save
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        let c = Color::new(102, 153, 255, 255);
        assert_eq!(c.to_hex(), "#6699ff");
        let c = Color::new(0,0,0,0);
        assert_eq!(c.to_hex(), "#000000");
    }

    #[test]
    fn output_colors() {
        let size = 512;
        let mut img = Image::new(size, size);
        for y in 0..size {
            for x in 0..size {
                let c: Color = Color::new((x * 255 / size) as u8, (y * 255 / size) as u8, 128, 255);
                img.set_pixel(x, y, c);
            }
        }
        img.save(&"graident.png".to_owned());
    }
}

use image::Color;

// use image::Image;

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub diffuse: f32,
    pub specular: f32,
    pub specular_n: i32,
    pub reflectivity: f32,
    // pub tint: bool,
    // Hack, should not be u32, should be image, but image can't be copied, so i need to rethink my structure
    pub texture: Option<u32>,
}

impl Material {
    pub fn new(
        color: Color,
        diffuse: f32,
        specular: f32,
        specular_n: i32,
        reflectivity: f32,
        // tint: bool,
        texture: Option<u32>,
    ) -> Material {
        return Material {
            color,
            diffuse,
            specular,
            specular_n,
            reflectivity,
            texture,
        };
    }
}

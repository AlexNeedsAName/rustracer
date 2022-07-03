use image::Color;

// use image::Image;

#[derive(Clone, Copy)]
pub enum Shading {
    FLAT,
    DIFFUSE,
}

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub reflectivity: f32,
    pub shading: Shading,
    // Hack, should not be u32, should be image, but image can't be copied, so i need to rethink my structure
    pub texture: Option<u32>,
}

impl Material {
    pub fn new(
        color: Color,
        reflectivity: f32,
        shading: Shading,
        texture: Option<u32>,
    ) -> Material {
        return Material {
            color: color,
            reflectivity: reflectivity,
            shading: shading,
            texture: texture,
        };
    }
}

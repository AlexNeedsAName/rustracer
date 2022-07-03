mod raytracer;

extern crate image;
extern crate matrix;

use image::Image;
use matrix::vector::{Point3D, Vector3D};
use raytracer::geometry::{Geometry, Sphere};
use raytracer::{Camera, Raytracer};

// Some cooridante ground rules:
// x is east/west, y is up/down, z is north/south

fn main() {
    let mut image = Image::new(512, 512);
    let camera = Camera {
        position: Vector3D::new([0.0, 0.0, 0.0]),
        look: Vector3D::new([0.0, 0.0, 1.0]),
        up: Vector3D::new([0.0, 1.0, 0.0]),
        fov: 60.0,
    };

    let mut scene: Vec<Box<dyn Geometry>> = Vec::new();
    scene.push(Box::new(Sphere {
        origin: Point3D::new([2.0, 2.0, 16.0]),
        radius: 5.3547,
    }));

    Raytracer::render(&camera, &scene, &mut image);

    image.save(&"output.png".to_owned());
}

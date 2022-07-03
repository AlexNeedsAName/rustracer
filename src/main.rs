mod raytracer;

extern crate image;
extern crate matrix;

use std::rc::Rc;
use image::{Color, Image};
use matrix::vector::{Point3D, Vector3D};
use raytracer::geometry::material::{Material, Shading};
use raytracer::geometry::{Geometry, Sphere};
use raytracer::{Camera, Raytracer};

// Some cooridante ground rules:
// x is east/west, y is up/down, z is north/south

fn main() {
    let mut image = Image::new(512, 512);
    let camera = Camera {
        position: Vector3D::new([0.0, 0.0, -16.0]),
        look: Vector3D::new([0.0, 0.0, 1.0]),
        up: Vector3D::new([0.0, 1.0, 0.0]),
        fov: 60.0,
    };

    let mut scene: Vec<Rc<dyn Geometry>> = Vec::new();
    scene.push(Rc::new(Sphere {
        origin: Point3D::new([0.0, 0.0, 16.0]),
        radius: 5.3547,
        material: Material::new(Color::new(0, 0, 255, 255), 0.0, Shading::DIFFUSE, None),
    }));
    scene.push(Rc::new(Sphere {
        origin: Point3D::new([10.0, 0.0, 16.0]),
        radius: 2.0,
        material: Material::new(Color::new(255, 0, 0, 255), 0.0, Shading::DIFFUSE, None),
    }));
    scene.push(Rc::new(Sphere {
        origin: Point3D::zero(),
        radius: f32::INFINITY,
        material: Material::new(Color::new(255, 255, 255, 255), 0.0, Shading::FLAT, None),
    }));

    let light = Point3D::new([2.0, 2.0, 2.0]);

    Raytracer::render(&camera, &scene, light, &mut image);

    let white = Color::new(255,255,255,255);
    println!("Color: {:?}", white * 0.75);
    println!("Color: {:?}", white * 0.5);
    println!("Color: {:?}", white * 0.25);
    println!("Color: {:?}", white * 0.0);

    image.save(&"output.png".to_owned());
}

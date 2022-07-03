extern crate image;
extern crate matrix;

use std::rc::Rc;

use image::{Color, Image};
use matrix::vector::{Point3D, Vector3D};

use raytracer::geometry::material::{Material, Shading};
use raytracer::geometry::{Geometry, Sphere};
use raytracer::{Camera, Raytracer};

use crate::raytracer::geometry::Triangle;

mod raytracer;

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

    let mirror = Rc::new(Material::new(
        Color::new(0, 0, 0, 255),
        1.0,
        Shading::FLAT,
        None,
    ));
    let white = Rc::new(Material::new(
        Color::new(255, 255, 255, 255),
        0.0,
        Shading::DIFFUSE,
        None,
    ));
    let blue = Rc::new(Material::new(
        Color::new(0, 0, 255, 255),
        0.0,
        Shading::DIFFUSE,
        None,
    ));
    let red = Rc::new(Material::new(
        Color::new(255, 0, 0, 255),
        0.0,
        Shading::DIFFUSE,
        None,
    ));
    let shiny_red = Rc::new(Material::new(
        Color::new(255, 0, 0, 255),
        0.1,
        Shading::DIFFUSE,
        None,
    ));
    let void = Rc::new(Material::new(
        Color::new(0, 0, 0, 255),
        0.0,
        Shading::FLAT,
        None,
    ));

    let mut scene: Vec<Rc<dyn Geometry>> = Vec::new();
    scene.push(Rc::new(Sphere {
        origin: Point3D::new([0.0, 0.0, 16.0]),
        radius: 2.0,
        material: Rc::clone(&mirror),
    }));
    scene.push(Rc::new(Sphere {
        origin: Point3D::new([3.0, -1.0, 14.0]),
        radius: 1.0,
        material: Rc::clone(&mirror),
    }));
    scene.push(Rc::new(Sphere {
        origin: Point3D::new([-3.0, -1.0, 14.0]),
        radius: 1.0,
        material: Rc::clone(&shiny_red),
    }));

    // The room contianing the spheres:
    //Back wall
    scene.push(Rc::new(Triangle {
        a: Point3D::new([-8.0, -2.0, 20.0]),
        b: Point3D::new([8.0, -2.0, 20.0]),
        c: Point3D::new([8.0, 10.0, 20.0]),
        material: Rc::clone(&blue),
    }));
    scene.push(Rc::new(Triangle {
        a: Point3D::new([-8.0, -2.0, 20.0]),
        b: Point3D::new([8.0, 10.0, 20.0]),
        c: Point3D::new([-8.0, 10.0, 20.0]),
        material: Rc::clone(&blue),
    }));

    // Floor
    scene.push(Rc::new(Triangle {
        a: Point3D::new([-8.0, -2.0, 20.0]),
        b: Point3D::new([8.0, -2.0, 10.0]),
        c: Point3D::new([8.0, -2.0, 20.0]),
        material: Rc::clone(&white),
    }));
    scene.push(Rc::new(Triangle {
        a: Point3D::new([-8.0, -2.0, 20.0]),
        b: Point3D::new([-8.0, -2.0, 10.0]),
        c: Point3D::new([8.0, -2.0, 10.0]),
        material: Rc::clone(&white),
    }));

    // Red Triangle on left
    scene.push(Rc::new(Triangle {
        a: Point3D::new([8.0, -2.0, 10.0]),
        b: Point3D::new([8.0, 10.0, 20.0]),
        c: Point3D::new([8.0, -2.0, 20.0]),
        material: Rc::clone(&red),
    }));

    // Background Color
    scene.push(Rc::new(Sphere {
        origin: Point3D::zero(),
        radius: f32::INFINITY,
        material: Rc::clone(&void),
    }));

    let light = Point3D::new([3.0, 5.0, 15.0]);

    Raytracer::render(&camera, &scene, light, &mut image, 20);

    image.save(&"output.png".to_owned());
}

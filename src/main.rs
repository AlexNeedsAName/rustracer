extern crate image;
extern crate matrix;
extern crate num_cpus;

use std::rc::Rc;

use image::{Color, Image};
use matrix::vector::{Point3D, Vector3D};

use raytracer::geometry::material::Material;
use raytracer::geometry::{Geometry, Sphere, Triangle};
use raytracer::Antialiasing::{Grid, Off};
use raytracer::{Camera, Raytracer};

mod raytracer;

// Some cooridante ground rules:
// x is east/west, y is up/down, z is north/south

fn main() {
    println!("Num Threads: {}", num_cpus::get());

    // let image = Image::new(2560, 1080);
    // let image = Image::new(7680, 7680);
    let image = Image::new(512, 512);

    let camera = Camera {
        position: Vector3D::new([0.0, 0.0, 0.0]),
        look: Vector3D::new([0.0, 0.0, 2.0]),
        up: Vector3D::new([0.0, 1.0, 0.0]),
        fov: 53.13010235,
    };

    let mirror = Rc::new(Material::new(
        Color::new(0, 0, 0, 255),
        0.0,
        1.0,
        1250,
        1.0,
        None,
    ));
    let white = Rc::new(Material::new(
        Color::new(255, 255, 255, 255),
        1.0,
        0.0,
        0,
        0.0,
        None,
    ));
    let blue = Rc::new(Material::new(
        Color::new(0, 0, 255, 255),
        1.0,
        0.0,
        0,
        0.0,
        None,
    ));
    let red = Rc::new(Material::new(
        Color::new(255, 0, 0, 255),
        0.5,
        0.0,
        0,
        0.0,
        None,
    ));
    let shiny_red = Rc::new(Material::new(
        Color::new(255, 0, 0, 255),
        1.0,
        0.5,
        50,
        0.1,
        None,
    ));
    let green = Rc::new(Material::new(
        Color::new(255, 0, 0, 255),
        0.5,
        0.0,
        0,
        0.0,
        None,
    ));
    let void = Rc::new(Material::new(
        Color::new(0, 0, 0, 255),
        0.0,
        0.0,
        0,
        0.0,
        None,
    ));

    let focus = Point3D::new([0.0, 0.0, 16.0]);

    let mut scene: Vec<Rc<dyn Geometry>> = Vec::new();
    scene.push(Rc::new(Sphere {
        origin: focus,
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

    // The room containing the spheres:
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

    let key_light = Point3D::new([3.0, 5.0, 15.0]);
    let tmp1 = key_light - focus;
    let tmp2 = Point3D::new([-tmp1.x(), tmp1.y(), tmp1.z()]);
    let fill_light = tmp2 + focus;
    let tmp3 = Point3D::new([-tmp1.x(), tmp1.y(), -tmp1.z()]);
    let back_light = tmp3 + focus;

    // scene.push(Rc::new(Sphere {
    //     origin: key_light,
    //     radius: 0.1,
    //     material: Rc::clone(&green),
    // }));
    // scene.push(Rc::new(Sphere {
    //     origin: fill_light,
    //     radius: 0.1,
    //     material: Rc::clone(&green),
    // }));
    // scene.push(Rc::new(Sphere {
    //     origin: back_light,
    //     radius: 0.1,
    //     material: Rc::clone(&green),
    // }));

    let mut lights: Vec<Point3D> = Vec::new();
    lights.push(key_light);
    lights.push(fill_light);
    lights.push(back_light);

    // let mut raytracer = Raytracer::new(&camera, image, Off);
    let mut raytracer = Raytracer::new(&camera, image, Grid(8));
    raytracer.render(&scene, &lights, 20);
    raytracer.save(&"output/output.png".to_owned());
}

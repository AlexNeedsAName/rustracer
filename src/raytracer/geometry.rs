extern crate image;
extern crate matrix;

use std::fmt;

pub mod material;

use material::Material;
use matrix::vector::Point3D;
use matrix::vector::Vector3D;

// Some cooridante ground rules:
// x is east/west, y is up/down, z is north/south

pub struct Rayhit {
    pub distance: f32,
    pub hit_position: Point3D,
    pub normal: Vector3D,
    pub material: Material,
}

impl Rayhit {
    pub fn new(dist: f32, hit_position: Point3D, normal: Vector3D, material: Material) -> Rayhit {
        return Rayhit {
            distance: dist,
            hit_position: hit_position,
            normal: normal,
            material: material,
        };
    }
}

pub struct Ray {
    pub direction: Vector3D,
    pub origin: Point3D,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3D {
        return self.origin + self.direction * t;
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("Ray({} -> {})", self.origin, self.direction))
    }
}

pub trait Geometry {
    fn intersect(&self, ray: &Ray, closest_dist: f32) -> Option<Rayhit>;
    fn normal(&self, position: Point3D) -> Vector3D;
}

pub struct Sphere {
    pub origin: Point3D,
    pub radius: f32,
    pub material: Material,
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray, closest_dist: f32) -> Option<Rayhit> {
        // We trivially hit an infinite sphere infinitely far away
        if f32::is_infinite(self.radius) {
            if !f32::is_infinite(closest_dist) {
                return None;
            } else {
                return Some(Rayhit::new(
                    f32::INFINITY,
                    Point3D::zero(),
                    ray.direction.clone(),
                    self.material,
                ));
            }
        }

        let d = ray.direction;
        let e = ray.origin;
        let c = self.origin;
        let r = self.radius;
        let discriminant = ((d * (e - c)) * (d * (e - c))) - (d * d) * ((e - c) * (e - c) - r * r);

        /*
        println!("Offset: {}", offset);
        println!("Ray Direction {}", ray.direction);
        println!("Ray Origin {}", ray.origin);
        println!("Sphere Center {}", self.origin);
        println!("Radius {}", self.radius);
        */

        if discriminant <= 0.0 {
            return None;
        }

        let root = discriminant.sqrt();
        let t1 = (-d * (e - c) + root) / (d * d);
        let t2 = (-d * (e - c) - root) / (d * d);

        //println!("Disc: {}", discriminant);
        //println!("t1: {}, t2: {}", t1, t2);

        if t1 < 0.0 && t2 < 0.0 {
            return None;
        } else if t1 < closest_dist && (t2 < 0.0 || t1 < t2) {
            let hit_pos = ray.at(t1);
            return Some(Rayhit::new(
                t1,
                hit_pos,
                self.normal(hit_pos),
                self.material,
            ));
        } else if t2 < closest_dist && (t1 < 0.0 || t2 < t1) {
            let hit_pos = ray.at(t2);
            return Some(Rayhit::new(
                t2,
                hit_pos,
                self.normal(hit_pos),
                self.material,
            ));
        } else {
            return None;
        }
    }

    fn normal(&self, _position: Point3D) -> Vector3D {
        return Vector3D::zero();
    }
}

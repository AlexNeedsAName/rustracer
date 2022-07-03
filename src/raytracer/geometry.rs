extern crate image;
extern crate matrix;

use std::fmt;

use matrix::vector::Point3D;
use matrix::vector::Vector3D;

// Some cooridante ground rules:
// x is east/west, y is up/down, z is north/south

pub struct Rayhit {
    pub distance: f32,
    pub hit_position: Point3D,
}

impl Rayhit {
    pub fn miss() -> Rayhit {
        return Rayhit {
            distance: -1.0,
            hit_position: Point3D::new([0.0, 0.0, 0.0]),
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
    fn intersect(&self, ray: &Ray) -> Rayhit;
    fn normal(&self, position: Point3D) -> Vector3D;
}

pub struct Sphere {
    pub origin: Point3D,
    pub radius: f32,
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Rayhit {
        // We trivially hit an infinite sphere infinitely far away
        if f32::is_infinite(self.radius) {
            return Rayhit {
                distance: f32::INFINITY,
                hit_position: Point3D::new([0.0, 0.0, 0.0]),
            };
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
            return Rayhit::miss();
        }

        let root = discriminant.sqrt();
        let t1 = (-d * (e - c) + root) / (d * d);
        let t2 = (-d * (e - c) - root) / (d * d);

        //println!("Disc: {}", discriminant);
        //println!("t1: {}, t2: {}", t1, t2);

        if t1 < 0.0 && t2 < 0.0 {
            return Rayhit::miss();
        } else if t2 < 0.0 || t1 < t2 {
            return Rayhit {
                distance: t1,
                hit_position: ray.at(t1),
            };
        } else if t1 < 0.0 || t2 < t1 {
            return Rayhit {
                distance: t2,
                hit_position: ray.at(t2),
            };
        } else {
            return Rayhit::miss();
        }
    }

    fn normal(&self, _position: Point3D) -> Vector3D {
        return Vector3D::new([0.0, 0.0, 0.0]);
    }
}

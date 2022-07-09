use std::fmt;
use std::rc::Rc;

use crate::matrix::vector::Point3D;
use crate::matrix::vector::Vector3D;

use crate::Color;
use material::Material;

pub mod material;

// Some coordinate ground rules:
// x is east/west, y is up/down, z is north/south

pub struct Lights {
    pub sources: Vec<Light>,
    pub total_intensity: f32,
}

impl Lights {
    pub fn new(sources: Vec<Light>) -> Lights {
        let mut intensity = 0.0;
        for light in &sources {
            intensity += light.intensity;
        }
        return Lights {
            sources,
            total_intensity: intensity,
        };
    }
}

pub struct Light {
    pub source: Point3D,
    pub color: Color,
    pub intensity: f32,
}

pub struct Rayhit {
    pub dist: f32,
    pub pos: Point3D,
    pub normal: Vector3D,
    pub material: Rc<Material>,
    pub obj: Rc<dyn Geometry>,
}

impl Rayhit {
    pub fn new(
        dist: f32,
        pos: Point3D,
        normal: Vector3D,
        material: Rc<Material>,
        obj: Rc<dyn Geometry>,
    ) -> Rayhit {
        return Rayhit {
            dist: dist,
            pos: pos,
            normal: normal,
            material: material,
            obj: obj,
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
    fn intersect(self: Rc<Self>, ray: &Ray, closest_dist: f32) -> Option<Rayhit>;
    fn normal(&self, position: Point3D) -> Vector3D;
}

pub struct Sphere {
    pub origin: Point3D,
    pub radius: f32,
    pub material: Rc<Material>,
}

impl Geometry for Sphere {
    fn intersect(self: Rc<Self>, ray: &Ray, closest_dist: f32) -> Option<Rayhit> {
        // We trivially hit an infinite sphere infinitely far away
        if f32::is_infinite(self.radius) {
            return if f32::is_finite(closest_dist) {
                None
            } else {
                Some(Rayhit::new(
                    f32::INFINITY,
                    ray.direction * f32::INFINITY,
                    -ray.direction,
                    Rc::clone(&self.material),
                    self,
                ))
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
            return None;
        }

        let root = discriminant.sqrt();
        let t1 = (-d * (e - c) + root) / (d * d);
        let t2 = (-d * (e - c) - root) / (d * d);

        //println!("Disc: {}", discriminant);
        //println!("t1: {}, t2: {}", t1, t2);

        return if t1 < 0.0 && t2 < 0.0 {
            None
        } else if t1 < closest_dist && (t2 < 0.0 || t1 < t2) {
            let hit_pos = ray.at(t1);
            Some(Rayhit::new(
                t1,
                hit_pos,
                self.normal(hit_pos),
                Rc::clone(&self.material),
                self,
            ))
        } else if t2 < closest_dist && (t1 < 0.0 || t2 < t1) {
            let hit_pos = ray.at(t2);
            Some(Rayhit::new(
                t2,
                hit_pos,
                self.normal(hit_pos),
                Rc::clone(&self.material),
                self,
            ))
        } else {
            None
        };
    }

    fn normal(self: &Sphere, position: Point3D) -> Vector3D {
        return (position - self.origin).normalized();
    }
}

pub struct Triangle {
    pub a: Point3D,
    pub b: Point3D,
    pub c: Point3D,
    pub material: Rc<Material>,
}

impl Geometry for Triangle {
    fn intersect(self: Rc<Self>, ray: &Ray, closest_dist: f32) -> Option<Rayhit> {
        let a = self.a.x() - self.b.x();
        let b = self.a.y() - self.b.y();
        let c = self.a.z() - self.b.z();
        let d = self.a.x() - self.c.x();
        let e = self.a.y() - self.c.y();
        let f = self.a.z() - self.c.z();
        let g = ray.direction.x();
        let h = ray.direction.y();
        let i = ray.direction.z();
        let j = self.a.x() - ray.origin.x();
        let k = self.a.y() - ray.origin.y();
        let l = self.a.z() - ray.origin.z();

        let m = a * (e * i - h * f) + b * (g * f - d * i) + c * (d * h - e * g);
        let beta = (j * (e * i - h * f) + k * (g * f - d * i) + l * (d * h - e * g)) / m;
        let gamma = (i * (a * k - j * b) + h * (j * c - a * l) + g * (b * l - k * c)) / m;
        let t = -(f * (a * k - j * b) + e * (j * c - a * l) + d * (b * l - k * c)) / m;

        return if t < 0.0
            || t > closest_dist
            || gamma < 0.0
            || gamma > 1.0
            || beta < 0.0
            || beta > 1.0 - gamma
        {
            None
        } else {
            let hit_pos = ray.at(t);
            Some(Rayhit::new(
                t,
                hit_pos,
                self.normal(hit_pos),
                Rc::clone(&self.material),
                self,
            ))
        };
    }

    fn normal(self: &Triangle, _position: Point3D) -> Vector3D {
        return (self.c - self.a).cross(&(self.b - self.a)).normalized();
    }
}

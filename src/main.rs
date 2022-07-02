extern crate image;
extern crate matrix;

use std::fmt;

use image::Color;
use image::Image;
use matrix::vector::Point3D;
use matrix::vector::Vector3D;

// Some cooridante ground rules:
// x is east/west, y is up/down, z is north/south

trait Geometry {
    fn intersect(&self, ray: &Ray) -> Rayhit;
    fn normal(&self, position: Point3D) -> Vector3D;
}

struct Ray {
    direction: Vector3D,
    origin: Point3D,
}

impl fmt::Display for Ray {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("Ray({} -> {})", self.origin, self.direction))
    }
}

impl Ray {
    /*    pub fn from_angles(yaw: f32, pitch: f32, &Vector3D origin) -> Ray {

            return Ray { direction: Vector3D::new([]), origin: origin };
        }

        pub fn from_xy()
    */

    pub fn trace(&self, scene: &Vec<Box<dyn Geometry>>) -> Color {
        let mut closest_hit = Rayhit::miss();
        for object in scene.iter() {
            let hit = object.intersect(self);
            if (closest_hit.distance == -1.0 && hit.distance > 0.0)
                || hit.distance < closest_hit.distance
            {
                closest_hit = hit;
            }
        }

        if closest_hit.distance <= 0.0 {
            return Color {
                r: 128,
                g: 0,
                b: 0,
                a: 255,
            };
        } else {
            return Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            };
        }
    }

    pub fn at(&self, t: f32) -> Point3D {
        return self.origin + self.direction * t;
    }
}

struct Rayhit {
    pub distance: f32,
    pub hit_position: Point3D,
}

impl Rayhit {
    fn miss() -> Rayhit {
        return Rayhit {
            distance: -1.0,
            hit_position: Point3D::new([0.0, 0.0, 0.0]),
        };
    }
}

struct Camera {
    pub position: Vector3D, // The position of the camera in 3D space
    pub look: Vector3D,     // The direction to look
    pub up: Vector3D,       // Which direction is up on the screen. Must be orthagonal to look
    pub fov: f32,           // FOV of the resulting image.
}

struct Raytracer {
    //    image: Image,
    //    cam: Camera,
}

struct Sphere {
    origin: Point3D,
    radius: f32,
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
        let R = self.radius;
        let discriminant = ((d * (e - c)) * (d * (e - c))) - (d * d) * ((e - c) * (e - c) - R * R);

        /*
        println!("Disc: {}", discriminant);
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

    fn normal(&self, position: Point3D) -> Vector3D {
        return Vector3D::new([0.0, 0.0, 0.0]);
    }
}

impl Raytracer {
    pub fn trace(camera: &Camera, scene: &Vec<Box<dyn Geometry>>, image: &mut Image) {
        let right = camera.look.cross(&camera.up).scale(-1.0).normalized();
        let up = right.cross(&camera.look).scale(-1.0).normalized(); // Ensure the up vector is orthagonal to our look direction

        println!("right: {}", right);
        println!("up: {}", up);

        let distance = camera.look.norm();
        let half_plane_width = distance * f32::tan(camera.fov.to_radians());
        let half_plane_height =
            half_plane_width * image.get_height() as f32 / image.get_width() as f32;

        let _top_left = camera.position + camera.look - up * half_plane_height - right * half_plane_width;
        let top_left = Vector3D::new([-0.407716, 0.407716, 0.817028]);

        println!(
            "half_plane_height: {}, width: {}",
            half_plane_height, half_plane_width
        );

        println!("Top Left: {}", top_left);

        let _step_x = right * (2.0 * half_plane_width / image.get_width() as f32);
        let _step_y = up * (2.0 * half_plane_height / image.get_height() as f32);

        let ahh = 0.001592640625;
        let step_x = Vector3D::new([ahh, 0.0, 0.0]);
        let step_y = Vector3D::new([0.0, -ahh, 0.0]);

        println!("step x: {}", step_x);
        println!("step y: {}", step_y);

        let mut row_start = top_left;
        let mut ray_direction = row_start;
        for y in 0..image.get_height() {
            ray_direction = row_start;
            for x in 0..image.get_width() {
                let ray = Ray {
                    direction: ray_direction.normalized(),
                    origin: camera.position,
                };
                image.set_pixelu32(x, y, ray.trace(scene));
                ray_direction = ray_direction + step_x;
            }
            row_start = row_start + step_y;
        }

        println!("Bottom Right: {}", ray_direction);
    }
}

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

    Raytracer::trace(&camera, &scene, &mut image);

    Ray {
        direction: Vector3D::new([0.0, 0.0, 1.0]),
        origin: Point3D::new([0.0, 0.0, 0.0]),
    }
    .trace(&scene);

    image.save(&"output.png".to_owned());
}

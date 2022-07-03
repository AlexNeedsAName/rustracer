pub mod geometry;

extern crate image;
extern crate matrix;

use geometry::Geometry;
use geometry::Ray;
use geometry::Rayhit;
use image::Color;
use image::Image;
use matrix::vector::Vector3D;
use more_asserts as ma;

// Some cooridante ground rules:
// x is east/west, y is up/down, z is north/south

pub struct Raytracer {
    //    image: Image,
    //    cam: Camera,
}

pub struct Camera {
    pub position: Vector3D, // The position of the camera in 3D space
    pub look: Vector3D,     // The direction to look
    pub up: Vector3D,       // Which direction is up on the screen. Must be orthagonal to look
    pub fov: f32,           // FOV of the resulting image.
}

impl Raytracer {
    pub fn trace(ray: Ray, scene: &Vec<Box<dyn Geometry>>) -> Color {
        let mut closest_hit: Option<Rayhit> = None;
        let mut closest_dist = f32::INFINITY;
        for object in scene.iter() {
            match object.intersect(&ray, closest_dist) {
                Some(hit) => {
                    closest_dist = hit.distance;
                    closest_hit = Some(hit);
                }
                None => {}
            }
        }

        match closest_hit {
            Some(hit) => hit.material.color,
            None => Color::new(0, 0, 0, 0),
        }
    }

    pub fn render(camera: &Camera, scene: &Vec<Box<dyn Geometry>>, image: &mut Image) {
        let right = camera.look.cross(&camera.up).scale(-1.0).normalized();
        let up = right.cross(&camera.look).scale(-1.0).normalized(); // Ensure the up vector is orthagonal to our look direction

        println!("right: {}", right);
        println!("up: {}", up);

        let distance = camera.look.norm();
        let half_plane_width = distance * f32::tan(camera.fov.to_radians());
        let half_plane_height =
            half_plane_width * image.get_height() as f32 / image.get_width() as f32;

        let _top_left =
            camera.position + camera.look - up * half_plane_height - right * half_plane_width;
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
                image.set_pixelu32(x, y, Raytracer::trace(ray, scene));
                ray_direction = ray_direction + step_x;
            }
            row_start = row_start + step_y;
        }

        println!("Bottom Right: {}", ray_direction);
    }
}

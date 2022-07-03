extern crate image;
extern crate matrix;

use std::rc::Rc;

use image::Color;
use image::Image;
use matrix::vector::Point3D;
use matrix::vector::Vector3D;

use geometry::material::Shading;
use geometry::Geometry;
use geometry::Ray;
use geometry::Rayhit;

pub mod geometry;

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
    pub fn trace(
        ray: &Ray,
        scene: &Vec<Rc<dyn Geometry>>,
        light: Point3D,
        depth: u32,
        ignore: Option<Rc<dyn Geometry>>,
    ) -> Color {
        // if depth < 0 {
        //     return Color::new(0, 0, 0, 0);
        // }
        let mut closest_hit: Option<Rayhit> = None;
        let mut closest_dist = f32::INFINITY;
        for object in scene.iter() {
            match &ignore {
                Some(ignore) => {
                    if (&**ignore as *const dyn Geometry as *const ())
                        == (&**object as *const dyn Geometry as *const ())
                    {
                        continue;
                    }
                }
                None => {}
            }
            match Rc::clone(object).intersect(ray, closest_dist) {
                Some(hit) => {
                    closest_dist = hit.dist;
                    closest_hit = Some(hit);
                }
                None => {}
            }
        }

        match closest_hit {
            Some(hit) => {
                let mut color = match hit.material.shading {
                    Shading::FLAT => hit.material.color,
                    Shading::DIFFUSE => {
                        let to_light = light - hit.pos;
                        let dist_to_light = to_light.norm();
                        let to_light = to_light * (1.0 / dist_to_light);

                        let min = 0.0;

                        let mut brightness = hit.normal * to_light;
                        if brightness < min {
                            brightness = min;
                        } else if hit.dist.is_finite() {
                            // Add shadows
                            let min_shade = min / brightness;
                            let ray_to_light = Ray {
                                direction: to_light,
                                origin: hit.pos,
                            };

                            let mut light_amount = 1.0;
                            for object in scene.iter() {
                                // Don't let an object cast a shadow on itself
                                if (&*hit.obj as *const dyn Geometry as *const ())
                                    == (&**object as *const dyn Geometry as *const ())
                                {
                                    continue;
                                }
                                match Rc::clone(object).intersect(&ray_to_light, dist_to_light) {
                                    Some(shadow_hit) => {
                                        light_amount *= 1.0 - shadow_hit.material.color.a;
                                        if light_amount <= min_shade {
                                            light_amount = min_shade;
                                            break;
                                        }
                                    }
                                    None => {}
                                }
                            }
                            brightness *= light_amount;
                            // println!("brightness: {}", brightness);
                        }

                        hit.material.color * brightness
                    }
                };
                // Send a ray through if it's transparent
                if color.a < 1.0 {
                    let passthrough_color = Raytracer::trace(
                        &Ray {
                            direction: ray.direction,
                            origin: hit.pos,
                        },
                        scene,
                        light,
                        depth - 1,
                        Some(hit.obj)
                    );
                    color = color.overlay(passthrough_color);
                }
                color
            }
            None => Color::new(0, 0, 0, 0),
        }
    }

    pub fn render(cam: &Camera, scene: &Vec<Rc<dyn Geometry>>, light: Point3D, img: &mut Image) {
        let right = cam.look.cross(&cam.up).scale(-1.0).normalized();
        let up = right.cross(&cam.look).scale(-1.0).normalized(); // Ensure the up vector is orthagonal to our look direction

        println!("right: {}", right);
        println!("up: {}", up);

        let distance = cam.look.norm();
        let half_plane_width = distance * f32::tan(cam.fov.to_radians());
        let half_plane_height = half_plane_width * img.get_height() as f32 / img.get_width() as f32;

        let _top_left = cam.position + cam.look - up * half_plane_height - right * half_plane_width;
        let top_left = Vector3D::new([-0.407716, 0.407716, 0.817028]);

        println!(
            "half_plane_height: {}, width: {}",
            half_plane_height, half_plane_width
        );

        println!("Top Left: {}", top_left);

        let _step_x = right * (2.0 * half_plane_width / img.get_width() as f32);
        let _step_y = up * (2.0 * half_plane_height / img.get_height() as f32);

        let ahh = 0.001592640625;
        let step_x = Vector3D::new([ahh, 0.0, 0.0]);
        let step_y = Vector3D::new([0.0, -ahh, 0.0]);

        println!("step x: {}", step_x);
        println!("step y: {}", step_y);

        let mut row_start = top_left;
        let mut ray_direction = row_start;
        for y in 0..img.get_height() {
            ray_direction = row_start;
            for x in 0..img.get_width() {
                let ray = Ray {
                    direction: ray_direction.normalized(),
                    origin: cam.position,
                };
                img.set_pixelu32(x, y, Raytracer::trace(&ray, scene, light, 1, None));
                ray_direction = ray_direction + step_x;
            }
            row_start = row_start + step_y;
        }

        println!("Bottom Right: {}", ray_direction);
    }
}

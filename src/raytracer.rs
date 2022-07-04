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
        reflections: u32,
        ignore: Option<Rc<dyn Geometry>>,
    ) -> Color {
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

                        let min = 0.2;

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
                                        // Some light passes through transparent objects
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
                        reflections,
                        Some(Rc::clone(&hit.obj)),
                    );
                    color = color.overlay(passthrough_color);
                }
                // Do reflections
                if hit.material.reflectivity > 0.0 {
                    let reflect = ray.direction - hit.normal * (ray.direction * hit.normal) * 2.0;
                    if reflections > 0 {
                        // println!("Reflecting");
                        color = Raytracer::trace(
                            &Ray {
                                direction: reflect,
                                origin: hit.pos,
                            },
                            scene,
                            light,
                            reflections - 1,
                            Some(Rc::clone(&hit.obj)),
                        )
                        .average(color, hit.material.reflectivity)
                    } else {
                        println!("Reflection limit reached");
                    };
                }
                color
            }
            None => Color::new(0, 0, 0, 0),
        }
    }

    pub fn get_ray(cam: &Camera, img: &Image, x: u32, y: u32, width: f32, height: f32) -> Ray {
        let right = cam.look.cross(&cam.up).scale(-1.0).normalized();
        let up = right.cross(&cam.look).scale(-1.0).normalized();
        let center = cam.position + cam.look;

        return Ray {
            origin: cam.position,
            direction: center + right * (width * (2.0 * x as f32 / img.get_width() as f32 - 1.0))
                - up * (height * (2.0 * y as f32 / img.get_height() as f32 - 1.0)),
        };
    }

    pub fn render(
        cam: &Camera,
        scene: &Vec<Rc<dyn Geometry>>,
        light: Point3D,
        img: &mut Image,
        reflections: u32,
    ) {
        let distance = cam.look.norm();
        let plane_width = distance * f32::tan(cam.fov.to_radians() / 2.0);
        let plane_height = plane_width * img.get_height() as f32 / img.get_width() as f32;

        for y in 0..img.get_height() {
            for x in 0..img.get_width() {
                img.set_pixelu32(
                    x,
                    y,
                    Raytracer::trace(
                        &Raytracer::get_ray(cam, img, x, y, plane_width, plane_height),
                        scene,
                        light,
                        reflections,
                        None,
                    ),
                );
            }
        }
    }
}

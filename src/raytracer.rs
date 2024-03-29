// extern crate num_cpus;
// extern crate rayon;

use std::rc::Rc;
// use std::thread;

use crate::image::Color;
use crate::image::Image;
use crate::matrix::vector::Point3D;
use crate::matrix::vector::Vector3D;

use crate::raytracer::geometry::Lights;
use geometry::Geometry;
use geometry::Ray;
use geometry::Rayhit;

pub mod geometry;

// Some coordinate ground rules:
// x is east/west, y is up/down, z is north/south

const AMBIENT: f32 = 0.2;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Antialiasing {
    Off,
    Grid(u32),
}

pub struct Raytracer {
    origin: Point3D,
    center: Point3D,
    up: Vector3D,
    right: Vector3D,
    plane_width: f32,
    plane_height: f32,
    img: Image,
    aa: Antialiasing,
}

pub struct Camera {
    pub position: Vector3D, // The position of the camera in 3D space
    pub look: Vector3D,     // The direction to look
    pub up: Vector3D,       // Which direction is up on the screen. Must be orthagonal to look
    pub fov: f32,           // FOV of the resulting image.
}

pub fn clamp(input: f32) -> f32 {
    return if input < 0.0 {
        0.0
    } else if input > 1.0 {
        1.0
    } else {
        input
    };
}

impl Raytracer {
    pub fn new(cam: &Camera, img: Image, aa: Antialiasing) -> Raytracer {
        let right = cam.look.cross(&cam.up).scale(-1.0).normalized();
        let up = right.cross(&cam.look).scale(-1.0).normalized();
        let center = cam.position + cam.look;
        let distance = cam.look.norm();
        let plane_width = distance * f32::tan(cam.fov.to_radians() / 2.0);
        let plane_height = plane_width * img.get_height() as f32 / img.get_width() as f32;
        return Raytracer {
            origin: cam.position,
            right,
            up,
            center,
            plane_width,
            plane_height,
            img,
            aa,
        };
    }

    pub fn shade(
        ray: &Ray,
        hit: &Rayhit,
        scene: &Vec<Rc<dyn Geometry>>,
        lights: &Lights,
        reflections: u32,
    ) -> (Color, u32) {
        let mut color = Color::new(0, 0, 0, 0);
        let mut ray_count = 1;

        let reflect = ray.direction - hit.normal * (ray.direction * hit.normal) * 2.0;
        for light_source in &lights.sources {
            let to_light = light_source.source.clone() - hit.pos;
            let dist_to_light = to_light.norm();
            let to_light = to_light * (1.0 / dist_to_light);
            let ray_to_light = Ray {
                direction: to_light,
                origin: hit.pos,
            };

            let half_angle = (to_light - ray.direction).normalized();

            let mut light_amount = 1.0;
            // How much of the light reaches the hit position?
            if hit.dist.is_finite() {
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
                            if light_amount <= 0.0 {
                                light_amount = 0.0;
                                break;
                            }
                        }
                        None => {}
                    }
                }
            } else {
                color = hit.material.color;
                continue;
            }

            let mixed_color = light_source.color * hit.material.color;

            // TODO: Make this section look less gross
            let light_ambient = mixed_color * AMBIENT; //TODO: Make a parameter for the raytracer.
            let light_diffuse =
                mixed_color * (clamp(hit.normal * to_light) * light_amount * hit.material.diffuse);
            let light_specular = light_source.color
                * (f32::powi(clamp(hit.normal * half_angle), hit.material.specular_n)
                    * light_amount
                    * hit.material.specular);

            color =
                color + ((light_ambient + light_diffuse + light_specular) * light_source.intensity)
        }

        assert_ne!(reflections, 0);
        let light_reflected = if reflections > 0 && hit.material.reflectivity > 0.0 {
            let (reflected_color, reflected_rays) = Raytracer::trace(
                &Ray {
                    direction: reflect,
                    origin: hit.pos,
                },
                scene,
                lights,
                reflections - 1,
                Some(Rc::clone(&hit.obj)),
            );
            ray_count += reflected_rays;
            reflected_color
        } else {
            hit.material.color
        } * hit.material.reflectivity;
        let light_transparent = if hit.material.color.a < 1.0 {
            let (passthrough_color, passthrough_rays) = Raytracer::trace(
                &Ray {
                    direction: ray.direction,
                    origin: hit.pos,
                },
                scene,
                lights,
                reflections,
                Some(Rc::clone(&hit.obj)),
            );
            ray_count += passthrough_rays;
            passthrough_color
        } else {
            Color::new(0, 0, 0, 0)
        } * (1.0 - hit.material.color.a);
        // color = color.overlay(passthrough_color);
        // println!("light intensity: {}", total_intensity);

        return (
            color * (1.0 / lights.total_intensity) + light_reflected + light_transparent,
            ray_count,
        );
    }

    pub fn trace(
        ray: &Ray,
        scene: &Vec<Rc<dyn Geometry>>,
        lights: &Lights,
        reflections: u32,
        ignore: Option<Rc<dyn Geometry>>,
    ) -> (Color, u32) {
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

        return match closest_hit {
            Some(hit) => Raytracer::shade(ray, &hit, scene, lights, reflections),
            None => (Color::new(0, 0, 0, 0), 1),
        };
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        return Ray {
            origin: self.origin,
            direction: self.center
                + self.right * (self.plane_width * (2.0 * x / self.img.get_width() as f32 - 1.0))
                - self.up * (self.plane_height * (2.0 * y / self.img.get_height() as f32 - 1.0)),
        };
    }

    pub fn render(&mut self, scene: &Vec<Rc<dyn Geometry>>, lights: &Lights, reflections: u32) {
        println!("Rendering Scene...");
        use std::time::Instant;
        let now = Instant::now();

        // let num_threads = num_cpus::get();
        // let thread_pool = rayon::ThreadPoolBuilder::new()
        //     .num_threads(num_threads)
        //     .build()
        //     .unwrap();

        // for (x,y) in (0..self.img.get_width()).flat_map(move |a| (0..self.img.get_height()).map(move |b| (a, b))) {
        //     self.render_pixel(x, y, scene, light, reflections);
        // }

        let mut ray_count = 0;

        for y in 0..self.img.get_height() {
            for x in 0..self.img.get_width() {
                ray_count = ray_count + self.render_pixel(x, y, scene, lights, reflections);
            }
        }

        let elapsed = now.elapsed();
        println!(
            "Render took {:.2?} and traced {:?} rays",
            elapsed, ray_count
        );
    }

    pub fn render_pixel(
        &mut self,
        x: u32,
        y: u32,
        scene: &Vec<Rc<dyn Geometry>>,
        lights: &Lights,
        reflections: u32,
    ) -> u32 {
        let (color, ray_count) = match &self.aa {
            Antialiasing::Off => Raytracer::trace(
                &self.get_ray(x as f32, y as f32),
                scene,
                lights,
                reflections,
                None,
            ),

            Antialiasing::Grid(size) => {
                let sub_step = 1.0 / *size as f32;
                let offset = -0.5 + sub_step * 0.5;
                let mut color = Color::new(0, 0, 0, 0);
                let mut ray_count = 0;
                for sub_x in 0..*size {
                    for sub_y in 0..*size {
                        let (sample, rays) = Raytracer::trace(
                            &self.get_ray(
                                x as f32 + offset + sub_step * sub_x as f32,
                                y as f32 + offset + sub_step * sub_y as f32,
                            ),
                            scene,
                            lights,
                            reflections,
                            None,
                        );
                        color = color + sample;
                        ray_count += rays;
                    }
                }
                (color * (1.0 / (size * size) as f32), ray_count)
            }
        };
        self.img.set_pixelu32(x, y, color);
        return ray_count;
    }

    pub fn save(&self, str: &String) {
        self.img.save(str);
    }
}

pub struct Anaglyph {
    left: Raytracer,
    right: Raytracer,
    img: Image,
}

#[allow(dead_code)]
impl Anaglyph {
    pub fn new(cam: &Camera, img: Image, aa: Antialiasing, ipd: f32) -> Anaglyph {
        let right_vector = cam.look.cross(&cam.up).scale(-1.0).normalized();
        let left_eye = Camera {
            position: cam.position - (right_vector * (ipd / 2.0)),
            look: cam.look,
            up: cam.up,
            fov: cam.fov,
        };
        let right_eye = Camera {
            position: cam.position + (right_vector * (ipd / 2.0)),
            look: cam.look,
            up: cam.up,
            fov: cam.fov,
        };
        let left = Raytracer::new(&left_eye, Image::new_like(&img), aa);
        let right = Raytracer::new(&right_eye, Image::new_like(&img), aa);
        return Anaglyph { left, right, img };
    }

    pub fn render(&mut self, scene: &Vec<Rc<dyn Geometry>>, lights: &Lights, reflections: u32) {
        self.left.render(scene, lights, reflections);
        self.right.render(scene, lights, reflections);
        let left_filter = Color::new(0, 255, 255, 255);
        let right_filter = Color::new(255, 0, 0, 255);
        for y in 0..self.img.get_height() {
            for x in 0..self.img.get_width() {
                let color = self.left.img.get_pixelu32(x, y).to_gray() * left_filter
                    + self.right.img.get_pixelu32(x, y).to_gray() * right_filter;
                self.img.set_pixelu32(x, y, color);
            }
        }
    }

    pub fn save(&self, str: &String) {
        self.img.save(str);
    }
}

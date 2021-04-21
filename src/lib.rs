pub mod camera;
pub mod colors;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

use camera::Camera;
use colors::write_color;
use hittable::HittableList;
#[allow(unused_imports)]
use material::{Dielectric, Lambertian, Metal};
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use vec3::{
    color::Color,
    point::{random_unit_vector, Point3},
    Vec3,
};

pub fn ray_color(r: Ray, world: &HittableList, depth: isize) -> Color {
    // if we've exceeded the ray bounce limit, no more light is computed
    if depth <= 0 {
        return Color::new([0., 0., 0.]);
    }
    // if hit, return that color
    if let Some(rec) = world.hit(&r, 0.001, std::f64::INFINITY) {
        if let Some((scattered_ray, attenuation)) = rec.mat_ptr.clone().unwrap().scatter(&r, &rec) {
            return ray_color(scattered_ray, &world, depth - 1) * attenuation;
        }
        return Color::new([0., 0., 0.]);
    }
    // else, background light
    let unit_direction = r.direction().normalize();
    let t = (unit_direction.y() + 1.0) * 0.5;
    Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
}

pub fn render() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    let R = (std::f64::consts::PI / 4.).cos();

    let material_left = Rc::new(Lambertian::new(Color::new([0., 0., 1.])));
    let material_right = Rc::new(Lambertian::new(Color::new([1., 0., 0.])));

    world.add(Rc::new(Sphere::new(
        Point3::new([-R, 0.0, -1.0]),
        R,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new([R, 0.0, -1.0]),
        R,
        material_right,
    )));

    // Camera
    let cam = Camera::new(90., aspect_ratio);

    // rand generator for antialiasing
    let mut rng = thread_rng();

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for h in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", h);
        for w in 0..image_width {
            let mut pixel_color = Color::new([0., 0., 0.]);
            for _s in 0..samples_per_pixel {
                let (u, v) = (
                    (w as f64 + rng.gen::<f64>()) / (image_width - 1) as f64,
                    (h as f64 + rng.gen::<f64>()) / (image_height - 1) as f64,
                );
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}

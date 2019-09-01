use std::fs::File;
use std::io::Write;
use std::rc::Rc;

use rand::Rng;

use raytracing::camera::Camera;
use raytracing::hit::{Hit, HitableList};
use raytracing::material::{Lambertian, Metal};
use raytracing::sphere::Sphere;
use raytracing::{Ray, Vec3};

const MAX_BOUNCE: u8 = 10;

fn main() {
    let mut f = File::create("ch-08.ppm").expect("couldn't open file");

    let width = 800;
    let height = 400;
    let num_samples = 20;

    let world = spheres();
    let camera = Camera::new();

    let mut rng = rand::thread_rng();

    write!(f, "P3\n{} {}\n255\n", width, height).expect("couldn't write header");

    for (y, x) in pixel_iter(height, width) {
            let mut avg_color = Vec3::origin();

            for _ in 0..num_samples {
                let xr: f64 = rng.gen();
                let yr: f64 = rng.gen();
                let u = (x as f64 + xr) / width as f64;
                let v = (y as f64 + yr) / height as f64;

                let r = camera.get_ray(u, v);
                //let p = r.point_at_param(2.0);
                avg_color += color(&r, &world, 0);
            }
            avg_color /= num_samples as f64;
            avg_color = Vec3::new(
                avg_color[0].sqrt(),
                avg_color[1].sqrt(),
                avg_color[2].sqrt(),
            );

            let ir = (255.99 * avg_color[0]) as u8;
            let ig = (255.99 * avg_color[1]) as u8;
            let ib = (255.99 * avg_color[2]) as u8;

            write!(f, "{} {} {}\n", ir, ig, ib).expect("unable to write pixel");
    }
    for y in (0..height).rev() {
        for x in 0..width {
        }
    }
}

fn spheres() -> HitableList {
    let h: Vec<Box<dyn Hit>> = vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3)),
        )),
    ];

    HitableList::with_vals(h)
}

fn color(r: &Ray, world: &HitableList, bounces: u8) -> Vec3 {
    match world.hit(r, 0.001, std::f64::MAX) {
        Some(h) => {
            if bounces >= MAX_BOUNCE {
                return Vec3::new(0.0, 0.0, 0.0);
            }
            match h.mat.scatter(&r, &h) {
                Some((scattered, attenuation)) => {
                    attenuation * color(&scattered, world, bounces + 1)
                }
                None => Vec3::new(0.0, 0.0, 0.0),
            }
        }
        None => {
            let unit_dir = Vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_dir.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn pixel_iter(height: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).rev().flat_map(move |y| {
        (0..width).map(move |x| {
            (y, x)
        })
    })
}

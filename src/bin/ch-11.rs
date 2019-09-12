use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use rand::Rng;
use rayon::prelude::*;

use raytracing::camera::Camera;
use raytracing::hit::{Hit, HitableList};
use raytracing::material::{Lambertian, Dielectric, Metal};
use raytracing::sphere::Sphere;
use raytracing::{Ray, Vec3};

const MAX_BOUNCE: u8 = 10;

fn main() {
    let mut f = File::create("ch-11.ppm").expect("couldn't open file");

    let width = 800;
    let height = 400;
    let num_samples = 200;

    let world = spheres();

    let aspect = width as f64/height as f64;
    let pos = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = (&pos - &look_at).len();
    let aperture = 2.0;
    let camera = Camera::new(pos, look_at, v_up, 20.0, aspect, aperture, focus_dist);

    write!(f, "P3\n{} {}\n255\n", width, height).expect("couldn't write header");

    let pixels: Vec<(usize, usize, Vec3)> = pixel_list(height, width).par_iter()
        .map(|(y, x)| {
            let mut rng = rand::thread_rng();

            let mut avg_color = Vec3::origin();
            for _ in 0..num_samples {
                let xr: f64 = rng.gen();
                let yr: f64 = rng.gen();
                let u = (*x as f64 + xr) / width as f64;
                let v = (*y as f64 + yr) / height as f64;

                let r = camera.get_ray(u, v);
                avg_color += color(&r, &world.clone(), 0);
            }
            avg_color /= num_samples as f64;
            avg_color = Vec3::new(
                avg_color[0].sqrt(),
                avg_color[1].sqrt(),
                avg_color[2].sqrt(),
            );

            (*y, *x, avg_color)
        }).collect();

    for (_y, _x, avg_color) in pixels {
        let ir = (255.99 * avg_color[0]) as u8;
        let ig = (255.99 * avg_color[1]) as u8;
        let ib = (255.99 * avg_color[2]) as u8;

        write!(f, "{} {} {}\n", ir, ig, ib).expect("unable to write pixel");
    }
}

fn spheres() -> HitableList {
    let h: Vec<Arc<dyn Hit>> = vec![
        Arc::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
        )),
        Arc::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        )),
        Arc::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)),
        )),
        Arc::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Arc::new(Dielectric::new(1.5))
            )),
        Arc::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Arc::new(Dielectric::new(1.5))
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

fn pixel_list(height: usize, width: usize) -> Vec<(usize, usize)> {
    (0..height).rev().flat_map(move |y| {
        (0..width).map(move |x| {
            (y, x)
        })
    }).collect()
}

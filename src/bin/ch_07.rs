use std::fs::File;
use std::io::Write;

use rand::Rng;

use raytracing::camera::Camera;
use raytracing::hit::{Hit, HitableList};
use raytracing::sphere::Sphere;
use raytracing::{Ray, Vec3};

const MAX_BOUNCE: u8 = 10;

fn main() {
    let mut f = File::create("ch-07.ppm").expect("couldn't open file");

    let width = 800;
    let height = 400;
    let num_samples = 20;

    let world = spheres();
    let camera = Camera::new();

    let mut rng = rand::thread_rng();

    write!(f, "P3\n{} {}\n255\n", width, height).expect("couldn't write header");

    for y in (0..height).rev() {
        for x in 0..width {
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
    }
}

fn spheres() -> HitableList {
    let h: Vec<Box<dyn Hit>> = vec![
        Box::new(Sphere::with_vals(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::with_vals(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        Box::new(Sphere::with_vals(Vec3::new(1.0, 0.0, -1.0), 0.2)),
    ];

    HitableList::with_vals(h)
}

fn color(r: &Ray, world: &HitableList, bounces: u8) -> Vec3 {
    if bounces >= MAX_BOUNCE {
        let unit_dir = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_dir.y() + 1.0);

        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }

    match world.hit(r, 0.001, std::f64::MAX) {
        Some(h) => {
            let target = &h.point + &h.normal + random_in_unit_sphere();
            0.5 * color(
                &Ray::with_values(h.point.clone(), &target - &h.point),
                world,
                bounces + 1,
            )
        }
        None => {
            let unit_dir = Vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_dir.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        if v.sq_len() >= 1.0 {
            return v;
        }
    }
}

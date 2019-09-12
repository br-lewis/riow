use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use rand::Rng;

use raytracing::camera::Camera;
use raytracing::hit::{Hit, HitableList};
use raytracing::material::Lambertian;
use raytracing::sphere::Sphere;
use raytracing::{Ray, Vec3};

fn main() {
    let mut f = File::create("ch-06.ppm").expect("couldn't open file");

    let width = 400;
    let height = 200;
    let num_samples = 100;

    let world = spheres();

    let aspect = width as f64/height as f64;
    let pos = Vec3::origin();
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(pos, look_at, v_up, 90.0, aspect);

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
                avg_color += color(&r, &world);
            }
            avg_color /= num_samples as f64;

            let ir = (255.99 * avg_color[0]) as u8;
            let ig = (255.99 * avg_color[1]) as u8;
            let ib = (255.99 * avg_color[2]) as u8;

            write!(f, "{} {} {}\n", ir, ig, ib).expect("unable to write pixel");
        }
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
    ];

    HitableList::with_vals(h)
}

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    match world.hit(r, 0.0, std::f64::MAX) {
        Some(h) => 0.5 * Vec3::new(h.normal.x() + 1.0, h.normal.y() + 1.0, h.normal.z() + 1.0),
        None => {
            let unit_dir = Vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_dir.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

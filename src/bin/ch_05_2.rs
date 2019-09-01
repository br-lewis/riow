
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

use raytracing::{Vec3, Ray};
use raytracing::hit::{HitableList, Hit};
use raytracing::sphere::Sphere;
use raytracing::material::Lambertian;

fn main() {
    let mut f = File::create("ch-05-2.ppm")
        .expect("couldn't open file");

    let width = 200;
    let height = 100;

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::origin();

    let world = spheres();

    write!(f, "P3\n{} {}\n255\n", width, height)
        .expect("couldn't write header");

    for y in (0..height).rev() {
        for x in 0..width {
            let u = x as f64 / width as f64;
            let v = y as f64 / height as f64;

            let r = Ray::with_values(origin.clone(), &lower_left + u * &horizontal + v * &vertical);
            let color = color(&r, &world);

            let ir = (255.99 * color[0]) as u8;
            let ig = (255.99 * color[1]) as u8;
            let ib = (255.99 * color[2]) as u8;

            write!(f, "{} {} {}\n", ir, ig, ib)
                .expect("unable to write pixel");
        }
    }
}

fn spheres() -> HitableList {
    let h: Vec<Box<dyn Hit>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))))),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))),
    ];

    HitableList::with_vals(h)
}


fn color(r: &Ray, world: &HitableList) -> Vec3 {
    match world.hit(r, 0.0, std::f64::MAX) {
        Some(h) => {
            0.5 * Vec3::new(h.normal.x() + 1.0, h.normal.y() + 1.0, h.normal.z() + 1.0)
        },
        None => {
            let unit_dir = Vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_dir.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

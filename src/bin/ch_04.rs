
use std::fs::File;
use std::io::Write;

use raytracing::{Vec3, Ray};

fn main() {
    let mut f = File::create("ch-04.ppm")
        .expect("couldn't open file");

    let width = 200;
    let height = 100;

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::origin();

    write!(f, "P3\n{} {}\n255\n", width, height)
        .expect("couldn't write header");

    for y in (0..height).rev() {
        for x in 0..width {
            let u = x as f64 / width as f64;
            let v = y as f64 / height as f64;

            let r = Ray::with_values(origin.clone(), &lower_left + u * &horizontal + v * &vertical);
            let color = color(&r);

            let ir = (255.99 * color[0]) as u8;
            let ig = (255.99 * color[1]) as u8;
            let ib = (255.99 * color[2]) as u8;

            write!(f, "{} {} {}\n", ir, ig, ib)
                .expect("unable to write pixel");
        }
    }
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(&oc, r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;

    discriminant > 0.0
}


fn color(r: &Ray) -> Vec3 {
    let circle_center = Vec3::new(0.0, 0.0, -1.0);
    if hit_sphere(&circle_center, 0.5, r) {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        let unit_dir = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

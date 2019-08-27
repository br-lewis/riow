
use std::fs::File;
use std::io::Write;

use raytracing::Vec3;

fn main() {
    let mut f = File::create("ch-02.ppm")
        .expect("couldn't open file");

    let width = 200;
    let height = 100;

    write!(f, "P3\n{} {}\n255\n", width, height)
        .expect("couldn't write header");

    for y in (0..height).rev() {
        for x in 0..width {
            let color = Vec3::with_values(
                x as f64 / width as f64,
                y as f64 / height as f64,
                0.2
            );

            let ir = (255.99 * color[0]) as u8;
            let ig = (255.99 * color[1]) as u8;
            let ib = (255.99 * color[2]) as u8;

            write!(f, "{} {} {}\n", ir, ig, ib)
                .expect("unable to write pixel");
        }
    }
}



use std::fs::File;
use std::io::Write;

fn main() {
    let mut f = File::create("ch-01.ppm")
        .expect("couldn't open file");

    let width = 200;
    let height = 100;

    write!(f, "P3\n{} {}\n255\n", width, height)
        .expect("couldn't write header");

    for y in (0..height).rev() {
        for x in 0..width {
            let r = x as f32 / width as f32;
            let g = y as f32 / height as f32;
            let b = 0.2;
            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

            write!(f, "{} {} {}\n", ir, ig, ib)
                .expect("couldn't write pixel value");
        }
    }
}

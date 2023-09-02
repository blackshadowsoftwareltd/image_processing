use std::path::PathBuf;

use image::{Pixel, Rgba, RgbaImage};

pub fn gradient_image() {
    let p = PathBuf::new();
    let p = p.join("src");
    let gradient_path = p.join("gradient");

    let mut img = RgbaImage::new(500, 500);

    let start = Rgba::from_slice(&[0, 128, 0, 0]);
    let end = Rgba::from_slice(&[255, 255, 255, 255]);

    image::imageops::vertical_gradient(&mut img, start, end);
    let path = gradient_path.join("vertical_gradient.png");
    img.save(path).unwrap();

    image::imageops::vertical_gradient(&mut img, end, start);
    let path = gradient_path.join("vertical_gradient_reverse.png");
    img.save(path).unwrap();

    image::imageops::horizontal_gradient(&mut img, start, end);
    let path = gradient_path.join("horizontal_gradient.png");
    img.save(path).unwrap();

    image::imageops::horizontal_gradient(&mut img, end, start);
    let path = gradient_path.join("horizontal_gradient_reverse.png");
    img.save(path).unwrap();
}

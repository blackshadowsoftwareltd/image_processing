use std::path::PathBuf;

use image::imageops::crop;

pub fn crop_image() {
    let path = PathBuf::from("assets/food.jpg");
    let save_path = PathBuf::from("src/crop");
    let mut img = image::open(path).unwrap().to_rgba8();

    let cropped = crop(&mut img, 100, 100, 1500, 1200);
    let path = save_path.join("crop.png");
    cropped.to_image().save(path).unwrap();
}

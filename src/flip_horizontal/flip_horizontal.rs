use std::path::PathBuf;

use image::imageops::flip_horizontal;

pub fn flip_horizontal_image() {
    let path = PathBuf::from("assets/food.jpg");
    let seve_path = PathBuf::from("src/flip_horizontal");

    let img = image::open(path).unwrap();
    let flipped = flip_horizontal(&img);

    let path = seve_path.join("flip_horizontal.png");
    flipped.save(path).unwrap();
}

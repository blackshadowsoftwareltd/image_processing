use std::path::PathBuf;

use image::imageops::invert;

pub fn inverts_image() {
    let path = PathBuf::from("assets/food.jpg");
    let save_path = PathBuf::from("src/inverts");

    let mut image = image::open(path).unwrap();
    invert(&mut image);
    image.save(save_path.join("inverts.png")).unwrap();
}

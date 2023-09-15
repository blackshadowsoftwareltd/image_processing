use std::path::PathBuf;

use image::imageops::rotate90;

pub fn rotate90_image() {
    let path = PathBuf::from("assets/pxfuel.jpg");
    let save_path = PathBuf::from("src/rotate90");

    let img = image::open(path).unwrap();
    let rotated = rotate90(&img);

    let path = save_path.join("rotated90.jpg");
    rotated.save(path).unwrap();
}

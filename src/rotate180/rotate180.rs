use std::path::PathBuf;

use image::imageops::rotate180;

pub fn rotate180_image() {
    let path = PathBuf::from("assets/pxfuel.jpg");
    let save_path = PathBuf::from("src/rotate180");

    let img = image::open(path).unwrap();
    let rotate = rotate180(&img);

    let path = save_path.join("rotate180.jpg");
    rotate.save(path).unwrap();
}

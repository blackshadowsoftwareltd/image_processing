use std::path::PathBuf;

use image::imageops::rotate180_in_place;

pub fn rotate180_in_place_image() {
    let path = PathBuf::from("assets/pxfuel.jpg");
    let save_path = PathBuf::from("src/rotate180_in_place");

    let mut img = image::open(path).expect("o");
    rotate180_in_place(&mut img);

    let path = save_path.join("rotate180_in_place.jpg");
    img.save(path).expect("f");
}

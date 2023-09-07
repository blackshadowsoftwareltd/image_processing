use std::path::PathBuf;

use image::imageops::huerotate;

pub fn huerotate_image() {
    let path = PathBuf::from("assets/food.jpg");
    let seve_path = PathBuf::from("src/huerotate");

    let image = image::open(path).unwrap();
    let hue_image = huerotate(&image, 90);

    hue_image.save(seve_path.join("huerotate.png")).unwrap();
}

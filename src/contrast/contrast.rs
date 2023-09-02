use std::path::PathBuf;

use image::imageops::contrast;

pub fn contrast_image() {
    let path = PathBuf::from("assets/food.jpg");
    let seve_path = PathBuf::from("src/contrast");

    let img = image::open(path).unwrap();
    let c = contrast(&img, -20.0);
    c.save(seve_path.join("contrast.png")).unwrap();
}

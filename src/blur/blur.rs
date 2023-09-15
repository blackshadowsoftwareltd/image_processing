use std::path::PathBuf;

use image::imageops::blur;

pub fn blur_image() {
    let path = PathBuf::from("assets/food.jpg");
    let save_path = PathBuf::from("src/blur");

    let img = image::open(path).unwrap();
    let sigma = 20.0;
    let blured_image = blur(&img, sigma);
    blured_image
        .save(save_path.join("blured_image.png"))
        .unwrap();
}

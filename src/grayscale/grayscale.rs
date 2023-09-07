use std::path::PathBuf;

use image::imageops::grayscale;

pub fn grayscale_image() {
    let path = PathBuf::from("assets/food.jpg");
    let seve_path = PathBuf::from("src/grayscale");

    let image = image::open(path).unwrap();
    let gray_image = grayscale(&image);
    let gray_image = image::imageops::brighten(&gray_image, -90); // incress & decress darkness & brightness in grayscale

    gray_image.save(seve_path.join("grayscale.png")).unwrap();
}

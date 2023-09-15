use std::path::PathBuf;

use image::imageops::brighten;

pub fn brighten_image() {
    let path = PathBuf::from("assets/food.jpg");
    let save_path = PathBuf::from("src/brighten");
    let img = image::open(path).unwrap();
    let b = brighten(&img, 50);
    b.save(save_path.join("brighten.png")).unwrap();
}

use std::path::PathBuf;

use image::imageops::{resize, FilterType};

pub fn resize_image() {
    let path = PathBuf::from("assets/food.jpg");
    let seve_path = PathBuf::from("src/resize");

    let img = image::open(path).unwrap();

    let resized = resize(&img.clone(), 400, 300, FilterType::Nearest);
    resized.save(seve_path.join("resized.png")).unwrap();

    let resized = resize(
        &img,
        img.width() * 1.2 as u32,
        img.height() * 1.2 as u32,
        FilterType::Triangle,
    );
    resized.save(seve_path.join("resized_large.png")).unwrap();
}

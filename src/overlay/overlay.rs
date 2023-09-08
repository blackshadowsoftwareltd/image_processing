use std::path::PathBuf;

use image::imageops::overlay;

pub fn overlay_image() {
    let path = PathBuf::from("assets/food.jpg");
    let top_path = PathBuf::from("assets/pxfuel.jpg");
    let seve_path = PathBuf::from("src/overlay");

    let mut bottom = image::open(path.clone()).unwrap().to_rgba8();
    let top = image::open(top_path).unwrap().to_rgba8();

    overlay(&mut bottom, &top, 100, 100);
    let path = seve_path.join("overlay.png");
    bottom.save(path).unwrap();
}

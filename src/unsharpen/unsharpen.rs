use std::path::PathBuf;

use image::imageops::{resize, unsharpen, FilterType};

pub fn unsharpen_image() {
    let path = PathBuf::from("assets/pxfuel.jpg");
    let save_path = PathBuf::from("src/unsharpen");

    let mut img = image::open(path).unwrap();
    resize(&img.clone(), 400, 300, FilterType::Nearest);

    for i in 1..6 {
        for j in 1..6 {
            let image = unsharpen(&mut img, -i as f32 * 20.0, j * 20);
            let path = save_path.join(format!("unsharpen_{:?}_{:?}.jpg", i, j));
            image.save(path).unwrap()
        }
    }
}

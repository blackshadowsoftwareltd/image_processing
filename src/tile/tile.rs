use std::path::PathBuf;

use image::{GenericImageView, RgbaImage};

pub fn tile_image() {
    let path = PathBuf::from("assets/food.jpg");
    let save_path = PathBuf::from("src/tile");

    let tile = image::open(path).unwrap();
    let mut img = RgbaImage::new(tile.dimensions().0, tile.dimensions().1);
    image::imageops::tile(&mut img, &tile);
    let path = save_path.join("tiled_wallpaper.png");
    img.save(path).unwrap();
}

pub fn tile_image_from_doc() {
    let path = PathBuf::from("assets/pxfuel.jpg");
    let save_path = PathBuf::from("src/tile");

    let mut img = RgbaImage::new(1920, 1080);
    let tile = image::open(path).unwrap();

    image::imageops::tile(&mut img, &tile);

    let path = save_path.join("tile_from_doc.jpg");
    img.save(path).unwrap();
}

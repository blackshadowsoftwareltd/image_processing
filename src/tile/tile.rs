use std::path::PathBuf;

use image::{GenericImageView, RgbaImage};

pub fn tile_image() {
    let path = PathBuf::from("assets/food.jpg");
    let seve_path = PathBuf::from("src/tile");

    let tile = image::open(path).unwrap();
    let mut img = RgbaImage::new(tile.dimensions().0, tile.dimensions().1);
    image::imageops::tile(&mut img, &tile);
    let path = seve_path.join("tiled_wallpaper.png");
    img.save(path).unwrap();
}

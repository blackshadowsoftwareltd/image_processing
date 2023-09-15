use std::path::PathBuf;

use image::imageops::{dither, ColorMap};

pub fn dither_image() {
    let path = PathBuf::from("assets/food.jpg");
    let save_path = PathBuf::from("src/dither");

    let mut img = image::open(path).unwrap().into_rgba8();
    let color_map = MyColorMap;

    dither(&mut img, &color_map);

    img.save(save_path.join("dither.png")).unwrap();
}
struct MyColorMap;
impl ColorMap for MyColorMap {
    type Color = image::Rgba<u8>;
    fn index_of(&self, color: &Self::Color) -> usize {
        // Calculate the luminance of the color
        let luminance = 0.299 * color[0] as f32 + 0.587 * color[1] as f32 + 0.114 * color[2] as f32;

        // Return the index of the closest color in the map
        if luminance < 128.0 {
            0 // black
        } else if luminance > 192.0 {
            1 // white
        } else if color[0] > color[1] {
            2 // red
        } else {
            3 // green
        }
    }
    // Change the parameter type to match the trait
    fn map_color(&self, pixel: &mut Self::Color) {
        // Get the index of the pixel from the index_of method
        let index = self.index_of(pixel);

        // Assign the pixel to the color corresponding to the index
        match index {
            0 => *pixel = image::Rgba([0, 0, 0, 255]),       // black
            1 => *pixel = image::Rgba([255, 255, 255, 255]), // white
            2 => *pixel = image::Rgba([255, 0, 0, 255]),     // red
            3 => *pixel = image::Rgba([0, 255, 0, 255]),     // green
            _ => panic!("Invalid index"),
        }
    }
}

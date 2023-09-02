use dither::dither::dither_image;

pub mod blur;
pub mod brighten;
pub mod contrast;
pub mod dither;
pub mod fractal;
pub mod gradient;
pub mod scaledown;
pub mod scaleup;
pub mod tile;

fn main() {
    dither_image();
}

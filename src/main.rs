use huerotate::huerotate::huerotate_image;

pub mod blur;
pub mod brighten;
pub mod contrast;
pub mod dither;
pub mod fractal;
pub mod gradient;
pub mod grayscale;
pub mod scaledown;
pub mod scaleup;
pub mod tile;
pub mod huerotate;

fn main() {
    huerotate_image();
}

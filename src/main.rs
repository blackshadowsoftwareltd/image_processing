use rotate90::rotate90::rotate90_image;

pub mod blur;
pub mod brighten;
pub mod contrast;
pub mod crop;
pub mod dither;
pub mod flip_horizontal;
pub mod fractal;
pub mod gradient;
pub mod grayscale;
pub mod huerotate;
pub mod inverts;
pub mod overlay;
pub mod resize;
pub mod rotate90;
pub mod scaledown;
pub mod scaleup;
pub mod tile;

fn main() {
    rotate90_image();
}

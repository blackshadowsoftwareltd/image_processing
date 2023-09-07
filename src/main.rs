use flip_horizontal::flip_horizontal::flip_horizontal_image;

pub mod blur;
pub mod brighten;
pub mod contrast;
pub mod dither;
pub mod flip_horizontal;
pub mod fractal;
pub mod gradient;
pub mod grayscale;
pub mod huerotate;
pub mod inverts;
pub mod scaledown;
pub mod scaleup;
pub mod tile;

fn main() {
    flip_horizontal_image();
}

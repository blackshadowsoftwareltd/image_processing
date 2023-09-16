use unsharpen::unsharpen::unsharpen_image;

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
pub mod rotate180;
pub mod rotate180_in_place;
pub mod rotate90;
pub mod scaledown;
pub mod scaleup;
pub mod tile;
pub mod unsharpen;

fn main() {
    unsharpen_image();
}

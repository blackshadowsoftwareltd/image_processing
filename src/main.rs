use blur::blur::blur_image;

pub mod blur;
pub mod fractal;
pub mod gradient;
pub mod scaledown;
pub mod scaleup;
pub mod tile;

fn main() {
    blur_image();
}

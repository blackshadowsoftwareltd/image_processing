use brighten::brighten::brighten_image;

pub mod blur;
pub mod brighten;
pub mod fractal;
pub mod gradient;
pub mod scaledown;
pub mod scaleup;
pub mod tile;

fn main() {
    brighten_image();
}

use fractal::fractal::fractal_image;
use gradient::gradient::gradient_image;
use scaledown::scaledown::scaledown_image;
use scaleup::scaleup::scaleup_image;
use tile::tile::tile_image;

pub mod fractal;
pub mod gradient;
pub mod scaledown;
pub mod scaleup;
pub mod tile;

fn main() {
    fractal_image();
    gradient_image();
    scaledown_image();
    scaleup_image();
    tile_image();
}

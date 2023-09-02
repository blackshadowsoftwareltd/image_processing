use fractal::fractal::fractal_image;
use gradient::gradient::gradient_image;
use scaledown::scaledown::scaledown_image;

pub mod fractal;
pub mod gradient;
pub mod scaledown;

fn main() {
    fractal_image();
    gradient_image();
    scaledown_image();
}

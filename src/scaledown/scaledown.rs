use image::imageops::FilterType;
use image::ImageFormat;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
use std::time::{Duration, Instant};
pub fn scaledown_image() {
    let path = PathBuf::from("assets/food.jpg");
    let save_path = PathBuf::from("src/scaledown");
    let img = image::open(path).unwrap();
    for &(name, filter) in [
        ("near", FilterType::Nearest),
        ("tri", FilterType::Triangle),
        ("cmr", FilterType::CatmullRom),
        ("gauss", FilterType::Gaussian),
        ("lcz2", FilterType::Lanczos3),
    ]
    .iter()
    {
        let timer = Instant::now();
        let scaled = img.resize(800, 800, filter);
        println!("Scaled by {} in {}", name, Elapsed::from(&timer));
        let path = save_path.join(format!("test-{}.png", name));
        let mut output = File::create(path).unwrap();
        scaled.write_to(&mut output, ImageFormat::Png).unwrap();
    }

    for size in &[400_u32, 600, 800] {
        let timer = Instant::now();
        let scaled = img.thumbnail(*size, *size);
        println!("Thumbnailed to {} in {}", size, Elapsed::from(&timer));
        let path = save_path.join(format!("test-thumb{}.png", size));
        let mut output = File::create(path).unwrap();
        scaled.write_to(&mut output, ImageFormat::Png).unwrap();
    }
}

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1_000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1_000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

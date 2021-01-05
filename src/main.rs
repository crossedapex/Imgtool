// imgtool/src/main.rs

use std::env;
use std::path::Path;

fn main() {
    let image_path = env::args().skip(1).next().unwrap(); //read argument passed to imgtool w/ args func from env module
    let path = Path::new(&image_path); //take string as path to image file and create path instance
    let img = image::open(path).unwrap(); //open image
    let rotated = img.rotate90(); //return image as rotaed 90 degrees
    rotated.save(path).unwrap(); //save rotated image
}

use image::{GenericImageView, ImageBuffer, ImageReader};


fn main() {
    println!("Hello, world!");

    let image = ImageReader::open("path").unwrap().decode().unwrap();
    image.view(x, y, width, height)
}

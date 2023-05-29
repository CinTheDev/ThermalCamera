//use std::fs::File;
//use std::io::Write;
use image::{RgbImage, Rgb};

pub fn write_rgb(path: &str, image: &[u8], width: usize, height: usize) {
    let file_suffix = path.split('.').last().expect("Unrecognised file suffix");

    match file_suffix.to_ascii_lowercase().as_str() {
        "png" => write_png(path, image, width as u32, height as u32),

        _ => panic!()
    }
}

/*
pub fn write_grayscale(path: &str, image: &[u8], width: usize, height: usize) {
    let file_suffix = path.split('.').last().expect("Unrecognised file suffix");

    match file_suffix.to_ascii_lowercase().as_str() {
        "pgm" => write_pgm(path, image, width, height),
        "png" => write_png_grayscale(path, image, width as u32, height as u32),

        _ => panic!()
    }
}

fn write_pgm(path: &str, image: &[u8], width: usize, height: usize) {
    // Raw image is graymap
    let mut file = File::create(path).unwrap();

    let err_msg = "Failed to write image to disk.";

    // Write header info
    file.write(b"P5\n").expect(err_msg);
    file.write(format!("{} {}\n", width, height).as_bytes()).expect(err_msg);
    file.write(b"255\n").expect(err_msg);

    // Write image contents in binary format
    file.write(image).expect(err_msg);
}
*/


fn write_png(path: &str, image: &[u8], width: u32, height: u32) {
    let mut img_png = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let index = (y * 3) * width + (x * 3);

            let r = image[index as usize + 0];
            let g = image[index as usize + 1];
            let b = image[index as usize + 2];

            img_png.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    img_png.save(path).unwrap();
}

/*
fn write_png_grayscale(path: &str, image: &[u8], width: u32, height: u32) {
    let mut img_png = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let col = image[index as usize];
            img_png.put_pixel(x, y, Rgb([col, col, col]));
        }
    }

    img_png.save(path).unwrap();
}
*/

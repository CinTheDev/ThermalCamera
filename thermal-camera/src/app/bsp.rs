use image::{RgbImage, Rgb};
use std::fs;
use chrono;

pub fn check_usb() -> bool {
    //let mut paths = fs::read_dir("/media/thermal-camera").unwrap().peekable();
    //return paths.peek().is_some();
    return false;
}

pub fn get_usb_path(filetype: String) -> String {
    return format!("{}/capture/{}.{}", get_usb_dir(), get_time(), filetype);
}

fn get_time() -> String {
    let t = chrono::offset::Local::now();
    let date = t.date_naive().format("%Y-%m-%d");
    let time = t.time().format("%H-%M-%S");
    
    let res = format!("{}_{}", date, time);
    return res;
}

fn get_usb_dir() -> String {
    // Simply return last directory
    let paths = fs::read_dir("/media/thermal-camera").unwrap();
    return paths.last().unwrap().unwrap().path().to_str().unwrap().to_string();
}

pub fn write_rgb(path: &str, image: &[u8], width: usize, height: usize) {
    let file_suffix = path.split('.').last().expect("Unrecognised file suffix");

    match file_suffix.to_ascii_lowercase().as_str() {
        "png" => write_png(path, image, width as u32, height as u32),

        _ => panic!()
    }
}

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

    let filename = path.split("/").last().unwrap();
    let without_file = path.replace(filename, "");

    fs::create_dir_all(without_file).unwrap();
    img_png.save(path).unwrap();
}

use image::{RgbImage, Rgb};
use std::fs;
use chrono;

pub fn check_usb() -> bool {
    let mut paths = fs::read_dir("/dev").unwrap();
    return paths.any(|val| {
        return val.as_ref().unwrap().file_name() == "sda1";
    });
}

pub fn get_usb_path() -> String {
    return format!("/media/usb0/thermal-camera/{}.png", get_time());
}

fn get_time() -> String {
    let t = chrono::offset::Local::now();
    let date = t.date_naive().format("%Y-%m-%d");
    let time = t.time().format("%H-%M-%S");
    
    let res = format!("{}_{}", date, time);
    return res;
}

pub fn write_png(file_path: &str, image: &[u8], width: u32, height: u32) {
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

    println!("Path: {}\nFull string: {}", get_path(&file_path.to_string()), &file_path);
    fs::create_dir_all(get_path(&file_path.to_string())).unwrap();
    img_png.save(file_path).unwrap();
}

fn get_path(file_path: &String) -> String {
    let mut parts = file_path.split_inclusive('/');

    parts.next();
    parts.next_back();
    
    let mut res = "".to_string();

    for p in parts {
        res += p;
    }

    return res;
}

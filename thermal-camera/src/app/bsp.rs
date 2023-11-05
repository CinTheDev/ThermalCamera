use image::{RgbImage, Rgb};
use std::fs;
use sys_mount;
use chrono;

pub fn check_usb() -> bool {
    let mut paths = fs::read_dir("/dev").unwrap();
    return paths.any(|val| {
        return val.as_ref().unwrap().file_name() == "sda1";
    });
}

pub fn get_usb_path() -> String {
    return format!("/mnt/usb/thermal-camera/{}.png", get_time());
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

    // TODO: Fix mount
    let mount_result = sys_mount::Mount::builder()
        .mount_autodrop("/dev/sda1", "/mnt/usb", sys_mount::UnmountFlags::DETACH);

    if mount_result.is_err() {
        println!("ERROR: Mount /dev/sda1 on /mnt/usb failed.\n{}", mount_result.err().unwrap());
        return;
    }

    fs::create_dir_all(get_path(&file_path.to_string())).unwrap();
    img_png.save(file_path).unwrap();
}

fn get_path(file_path: &String) -> String {
    let mut parts = file_path.split('/').rev();
    parts.next();
    
    let mut res = "".to_string();

    for p in parts {
        res += p;
    }

    return res;
}

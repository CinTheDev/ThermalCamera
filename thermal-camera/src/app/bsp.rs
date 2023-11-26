use image::{RgbImage, Rgb};
use std::{fs, io::{self, Write, Read}};
use chrono;
use super::{Opt, mlx};
use std::str::FromStr;

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

pub fn write_options(opt: Opt) -> io::Result<()> {
    let opt_string = opt.parse_to_string();
    let buf = opt_string.as_bytes();

    let mut f = fs::File::create("options.txt")?;
    f.write_all(buf)?;

    return Ok(());
}

pub fn read_options() -> Result<Opt, io::Error> {
    let f_response = fs::File::open("options.txt");
    if f_response.is_err() {
        return Err(f_response.unwrap_err());
    }

    let mut f = f_response.unwrap();
    
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    return Ok(Opt::parse_from_string(buf));
}

impl Opt {
    fn parse_to_string(&self) -> String {
        format!(
            "color:{}\n
            left_hand:{}\n",
            self.color_type.to_string(),
            self.left_handed.to_string()
        )
    }

    fn parse_from_string(s: String) -> Self {
        let options = s.split('\n');
        let mut res: Self = Self::default();

        for o in options {
            let mut words = o.split(':');
            let key = words.next();
            let val = words.next();

            if key.is_none() || val.is_none() {
                continue;
            }

            match key.unwrap() {
                "color" => res.color_type = mlx::ColorTypes::from_str(
                    val.unwrap()
                ).unwrap_or(mlx::ColorTypes::Hue),

                "left_hand" => res.left_handed = match val.unwrap() {
                    "true" => true,
                    "false" => false,
                    _ => false,
                },

                _ => ()
            }
        }

        return res;
    }
}

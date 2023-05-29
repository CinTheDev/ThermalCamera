use image::{RgbImage, Rgb};

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

    img_png.save(path).unwrap();
}

mod bsp_mlx;
mod mlx_image;

use super::bsp;

const PIXELS_WIDTH: usize = 32;
const PIXELS_HEIGHT: usize = 24;
const PIXEL_COUNT: usize = PIXELS_WIDTH * PIXELS_HEIGHT;

pub fn init() {
    bsp_mlx::init();
}

pub fn test() {
    let img = take_image();
    //bsp_mlx::write_image("./test.pgm", &img, PIXELS_WIDTH, PIXELS_HEIGHT);
    bsp::write_pgm("./test.pgm", &img, PIXELS_WIDTH, PIXELS_HEIGHT);
}

fn read_image() -> [u16; PIXEL_COUNT] {
    let mut img: [u16; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

    let subpage = bsp_mlx::read_value(0x8000) & 0x1;
    let mut offset = subpage;

    for _sub in 0..2 {
        wait_for_data();

        for row in 0..PIXELS_HEIGHT as u16 {
            for i in 0..(PIXELS_WIDTH/2) as u16 {
                let mut addr: u16 = row * PIXELS_WIDTH as u16;
                let pos: u16 = i * 2 + (row + offset) % 2;
    
                addr += pos;
    
                let meas = bsp_mlx::read_value(0x0400 + addr);
    
                img[addr as usize] = meas;
            }
        }

        offset += 1;
    }

    return img;
}

fn wait_for_data() {
    loop {
        let status_reg = bsp_mlx::read_value(0x8000);

        // If that bit is a 1, it's bigger than 0
        let new_data = status_reg & 0x8 > 0;

        if new_data { break }
    }

    let mut status_reg = bsp_mlx::read_value(0x8000);
    status_reg &= !0x8; // Clear that bit
    
    bsp_mlx::write(0x8000, status_reg);
}

fn take_image() -> [u8; PIXEL_COUNT] {
    let image_raw = read_image();
    let image_temperatures = bsp_mlx::evaluate_image(image_raw);
    return mlx_image::grayscale_test(&image_temperatures);
}

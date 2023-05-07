mod bsp_mlx;

const PIXELS_WIDTH: usize = 32;
const PIXELS_HEIGHT: usize = 24;
const PIXEL_COUNT: usize = PIXELS_WIDTH * PIXELS_HEIGHT;

pub fn test() {
    bsp_mlx::read(0x0400);
}

fn read_image() -> [u8; PIXEL_COUNT] {
    let mut img: [u8; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

    for i in 0..PIXEL_COUNT {
        let addr: u16 = (0x0400 + i) as u16;
        let val = bsp_mlx::read(addr);
        img[i] = (val / 256) as u8;
    }

    return img;
}

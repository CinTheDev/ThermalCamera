mod bsp_mlx;

const PIXELS_WIDTH: usize = 32;
const PIXELS_HEIGHT: usize = 24;
const PIXEL_COUNT: usize = PIXELS_WIDTH * PIXELS_HEIGHT;

pub fn test() {
    let mut control_reg = bsp_mlx::read_value(0x800D);
    control_reg &= !(0x80 | 0x100 | 0x200); // Reduce refresh rate to 0.5Hz
    bsp_mlx::write(0x800D, control_reg);


    let img = read_image();
    bsp_mlx::write_image("./test.pgm", &img, PIXELS_WIDTH, PIXELS_HEIGHT);
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

fn read_image() -> [u8; PIXEL_COUNT] {
    let mut img: [u8; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

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
    
                img[addr as usize] = (meas) as u8;
            }
        }

        offset += 1;
    }

    return img;
}

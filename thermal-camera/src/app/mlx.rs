mod bsp_mlx;
pub mod mlx_image;

pub const PIXELS_WIDTH: usize = 32;
pub const PIXELS_HEIGHT: usize = 24;
pub const PIXEL_COUNT: usize = PIXELS_WIDTH * PIXELS_HEIGHT;

pub const GRADIENT_WIDTH: usize = 1;
pub const GRADIENT_HEIGHT: usize = 256;
pub const GRADIENT_COUNT: usize = GRADIENT_WIDTH * GRADIENT_HEIGHT;

#[derive(Debug, Clone, Copy)]
pub enum ColorTypes {
    Gray,
    Cheap,
    Hue,
}

#[derive(Debug)]
pub struct TemperatureRead {
    pub temperature_grid: [f32; PIXEL_COUNT],
    pub min_temp: f32,
    pub max_temp: f32,
}

pub struct ImageRead {
    pub pixels: [u8; PIXEL_COUNT * 3],
    pub temperature_read: TemperatureRead,
}

pub fn take_image(color_type: &ColorTypes) -> Result<ImageRead, String> {
    let temperature_grid = read_temperatures()?;

    return Ok(mlx_image::color_image(&color_type, &temperature_grid));
}

pub fn get_scale(color_type: ColorTypes) -> [u8; GRADIENT_COUNT * 3] {
    return mlx_image::color_gradient(color_type);
}

pub fn set_framerate(val: u8) {
    let speed_val: u16 = val.min(7) as u16;

    let mlx_response = bsp_mlx::read_value(0x800D);

    if mlx_response.is_err() { return; }
    let mut ctrl_register_1 = mlx_response.unwrap();

    ctrl_register_1 &= 0b111_1_11_000_111_1111;
    ctrl_register_1 |= speed_val << 7;

    bsp_mlx::write(0x800D, ctrl_register_1).unwrap_or_else(|err| {
        println!("Framerate update failed: {}", err);
    });
}

pub fn read_temperatures() -> Result<TemperatureRead, String> {
    let image_raw = read_raw_image()?;
    let image_eval = bsp_mlx::evaluate_image(image_raw)?;

    let mut image_flip: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];

    let mut min_temp: f32 =  99999.0;
    let mut max_temp: f32 = -99999.0;

    for y in 0..PIXELS_HEIGHT {
        // Flip around vertical axis
        for x in 0..PIXELS_WIDTH {
            let x_flip = PIXELS_WIDTH - x - 1;

            let old_index = y * PIXELS_WIDTH + x;
            let new_index = y * PIXELS_WIDTH + x_flip;

            image_flip[new_index] = image_eval[old_index];

            if min_temp > image_flip[new_index] {
                min_temp = image_flip[new_index];
            }
            if max_temp < image_flip[new_index] {
                max_temp = image_flip[new_index];
            }
        }
    }

    return Ok(TemperatureRead {
        temperature_grid: image_flip,
        min_temp,
        max_temp,
    })
}

fn read_raw_image() -> Result<[u16; PIXEL_COUNT], String> {
    let mut img: [u16; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

    let subpage = bsp_mlx::read_value(0x8000)? & 0x1;
    let mut offset = subpage;

    for _sub in 0..2 {
        wait_for_data();

        for row in 0..PIXELS_HEIGHT as u16 {
            for i in 0..(PIXELS_WIDTH/2) as u16 {
                let mut addr: u16 = row * PIXELS_WIDTH as u16;
                let pos: u16 = i * 2 + (row + offset) % 2;
    
                addr += pos;
    
                img[addr as usize ] = bsp_mlx::read_value(0x0400 + addr)?;
            }
        }

        offset += 1;
    }

    return Ok(img);
}

fn wait_for_data() {
    let mut status_reg: u16;
    loop {
        let mlx_response = bsp_mlx::read_value(0x8000);

        if mlx_response.is_err() { return; }
        status_reg = mlx_response.unwrap();

        // If that bit is a 1, it's bigger than 0
        let new_data = status_reg & 0x8 > 0;

        if new_data { break }
    }

    //let mut status_reg = bsp_mlx::read_value(0x8000);
    status_reg &= !0x8; // Clear that bit
    
    bsp_mlx::write(0x8000, status_reg).unwrap_or_else(|err| {
        println!("Couldn't wait for data: {}", err);
    });
}

impl Default for TemperatureRead {
    fn default() -> Self {
        TemperatureRead {
            temperature_grid: [0.0; PIXEL_COUNT],
            min_temp: 0.0,
            max_temp: 0.0
        }
    }
}

impl Default for ImageRead {
    fn default() -> Self {
        ImageRead {
            pixels: [0x00; PIXEL_COUNT * 3],
            temperature_read: TemperatureRead::default()
        }
    }
}

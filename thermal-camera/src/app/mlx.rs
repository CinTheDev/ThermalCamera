use std::str::FromStr;

mod bsp_mlx;
pub mod mlx_image;

use bsp_mlx::{REGISTER_STATUS, REGISTER_CTRL, ADDRESS_RAM};

pub const PIXELS_WIDTH: usize = 32;
pub const PIXELS_HEIGHT: usize = 24;
pub const PIXEL_COUNT: usize = PIXELS_WIDTH * PIXELS_HEIGHT;

pub const GRADIENT_WIDTH: usize = 1;
pub const GRADIENT_HEIGHT: usize = 256;
pub const GRADIENT_COUNT: usize = GRADIENT_WIDTH * GRADIENT_HEIGHT;

#[derive(Debug, Clone, Copy)]
pub enum Framerates {
    Half = 0b000,
    One = 0b001,
    Two = 0b010,
    Four = 0b011,
    Eight = 0b100,
    Sixteen = 0b101,
    Thirtytwo = 0b110,
    Sixtyfour = 0b111,
}

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

#[derive(Debug)]
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

pub fn set_framerate(val: Framerates) {
    let speed_val: u16 = val as u16;

    let mlx_response = bsp_mlx::read_value(REGISTER_CTRL);

    if mlx_response.is_err() { return; }
    let mut ctrl_register_1 = mlx_response.unwrap();

    ctrl_register_1 &= 0b111_1_11_000_111_1111;
    ctrl_register_1 |= speed_val << 7;

    bsp_mlx::write(0x800D, ctrl_register_1).unwrap_or_else(|err| {
        println!("Framerate update failed: {}", err);
    });
}

pub fn read_framerate() -> Result<Framerates, String> {
    let ctrl_register = bsp_mlx::read_value(REGISTER_CTRL)?;
    let refresh_rate_raw = ((ctrl_register >> 7) & 0x7) as u8;
    return Ok(refresh_rate_raw.try_into().unwrap());
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

    let subpage = bsp_mlx::read_value(REGISTER_STATUS)? & 0x1;
    let mut offset = subpage;

    for _sub in 0..2 {
        wait_for_data();

        for row in 0..PIXELS_HEIGHT as u16 {
            for i in 0..(PIXELS_WIDTH/2) as u16 {
                let mut addr: u16 = row * PIXELS_WIDTH as u16;
                let pos: u16 = i * 2 + (row + offset) % 2;
    
                addr += pos;
    
                img[addr as usize ] = bsp_mlx::read_value(ADDRESS_RAM + addr)?;
            }
        }

        offset += 1;
    }

    return Ok(img);
}

fn wait_for_data() {
    let mut status_reg: u16;
    loop {
        let mlx_response = bsp_mlx::read_value(REGISTER_STATUS);

        if mlx_response.is_err() { return; }
        status_reg = mlx_response.unwrap();

        // If that bit is a 1, it's bigger than 0
        let new_data = status_reg & 0x8 > 0;

        if new_data { break }
    }

    status_reg &= !0x8; // Clear that bit
    
    bsp_mlx::write(0x8000, status_reg).unwrap_or(()); // Ignore errors
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

impl ToString for ColorTypes {
    fn to_string(&self) -> String {
        match self {
            ColorTypes::Gray => "Gray".into(),
            ColorTypes::Cheap => "Cheap".into(),
            ColorTypes::Hue => "Hue".into()
        }
    }
}

impl FromStr for ColorTypes {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gray" => Ok(ColorTypes::Gray),
            "cheap" => Ok(ColorTypes::Cheap),
            "hue" => Ok(ColorTypes::Hue),

            _ => Err("Unrecognised color type")
        }
    }
}

impl Framerates {
    pub fn increase(&self) -> Self {
        let current_val = *self as i8;
        let new_val = (current_val + 1).min(7) as u8;
        return new_val.try_into().unwrap();
    }

    pub fn decrease(&self) -> Self {
        let current_val = *self as i8;
        let new_val = (current_val - 1).max(0) as u8;
        return new_val.try_into().unwrap();
    }
}

impl TryFrom<u8> for Framerates {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0b000 => Ok(Framerates::Half),
            0b001 => Ok(Framerates::One),
            0b010 => Ok(Framerates::Two),
            0b011 => Ok(Framerates::Four),
            0b100 => Ok(Framerates::Eight),
            0b101 => Ok(Framerates::Sixteen),
            0b110 => Ok(Framerates::Thirtytwo),
            0b111 => Ok(Framerates::Sixtyfour),
            _ => Err(()),
        }
    }
}

impl FromStr for Framerates {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0.5" | "h" | "half" => Ok(Framerates::Half),
            "1" | "one" => Ok(Framerates::One),
            "2" | "two" => Ok(Framerates::Two),
            "4" | "four" => Ok(Framerates::Four),
            "8" | "eight" => Ok(Framerates::Eight),
            "16" | "sixteen" => Ok(Framerates::Sixteen),
            "32" | "thirtytwo" => Ok(Framerates::Thirtytwo),
            "64" | "sixtyfour" => Ok(Framerates::Sixtyfour),
            _ => Err("Unrecognised refresh rate. Only powers of 2 up to 64 are allowed")
        }
    }
}

impl ToString for Framerates {
    fn to_string(&self) -> String {
        match self {
            Framerates::Half => "0.5".into(),
            Framerates::One => "1".into(),
            Framerates::Two => "2".into(),
            Framerates::Four => "4".into(),
            Framerates::Eight => "8".into(),
            Framerates::Sixteen => "16".into(),
            Framerates::Thirtytwo => "32".into(),
            Framerates::Sixtyfour => "64".into(),
        }
    }
}

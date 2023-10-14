use super::{
    PIXEL_COUNT, GRADIENT_WIDTH, GRADIENT_HEIGHT, GRADIENT_COUNT,
    TemperatureRead, ImageRead, ColorTypes
};

fn grayscale_function(temp: f32, temp_min: f32, temp_max: f32) -> [u8; 3] {
    let value: f32 = (temp - temp_min) * (255.0 / temp_max);
    let value_byte: u8 = value.round().max(0.0).min(255.0) as u8;

    return [value_byte; 3];
}

fn rgb_cheap_function(temp: f32, temp_min: f32, temp_max :f32) -> [u8; 3] {
    // Calculate interpolation value t, it is always between 0 and 1
    let mut t = (temp - temp_min) / (temp_max - temp_min);
    t = t.max(0.0).min(1.0);

    // Use special formulas to get rgb colors, I created a really simple and cheap one here.
    // Usually the rgb colors will be between 0 and 1, but sometimes it overshoots
    // Later it's being clamped
    let factor = 2.0;

    let r = 0_f32.max(-factor * (1.0 - t) + 1.0);
    let b = 0_f32.max(-factor * t         + 1.0);

    let g = 1.0 - r - b;

    let r_byte = (r * 255.0).round().max(0.0).min(255.0) as u8;
    let g_byte = (g * 255.0).round().max(0.0).min(255.0) as u8;
    let b_byte = (b * 255.0).round().max(0.0).min(255.0) as u8;

    return [r_byte, g_byte, b_byte];
}

fn rgb_hue_function(temp: f32, temp_min: f32, temp_max :f32) -> [u8; 3] {
    // Calculate interpolation value t, it is always between 0 and 1
    let mut t = (temp - temp_min) / (temp_max - temp_min);
    t = t.max(0.0).min(1.0);

    let hue = (1.0 - t) * 275.0;

    let c: f32 = 1.0;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());

    let mut r: f32 = 0.0;
    let mut g: f32 = 0.0;
    let mut b: f32 = 0.0;
    
    if hue < 60.0 {
        r = c;
        g = x;
    }
    else if hue < 120.0 {
        r = x;
        g = c;
    }
    else if hue < 180.0 {
        g = c;
        b = x;
    }
    else if hue < 240.0 {
        g = x;
        b = c;
    }
    else if hue < 300.0 {
        r = x;
        b = c;
    }
    else if hue < 360.0 {
        r = c;
        b = x;
    }

    let r_byte = (r * 255.0).round() as u8;
    let g_byte = (g * 255.0).round() as u8;
    let b_byte = (b * 255.0).round() as u8;

    return [r_byte, g_byte, b_byte];
}

pub fn color_image(color_type: ColorTypes, temperatures: TemperatureRead, temp_min: f32, temp_max: f32) -> ImageRead {
    let mut res_pixels: [u8; PIXEL_COUNT * 3] = [0x00; PIXEL_COUNT * 3];

    for i in 0..PIXEL_COUNT {
        let index = i * 3;
        let temp = temperatures.temperature_grid[i];

        let color = match color_type {
            ColorTypes::Gray => grayscale_function(temp, temp_min, temp_max),
            ColorTypes::Cheap => rgb_cheap_function(temp, temp_min, temp_max),
            ColorTypes::Hue => rgb_hue_function(temp, temp_min, temp_max),
        };

        res_pixels[index..index+3].copy_from_slice(&color);
    }

    let mut gradient: [u8; GRADIENT_COUNT * 3] = [0x00; GRADIENT_COUNT * 3];
    for y in 0..GRADIENT_HEIGHT {
        let t = y as f32 / GRADIENT_HEIGHT as f32;
        let temp = temp_min + t * (temp_max - temp_min);
        let color = match color_type {
            ColorTypes::Gray => grayscale_function(temp, temp_min, temp_max),
            ColorTypes::Cheap => rgb_cheap_function(temp, temp_min, temp_max),
            ColorTypes::Hue => rgb_hue_function(temp, temp_min, temp_max),
        };

        for x in 0..GRADIENT_WIDTH {
            let index = y * GRADIENT_WIDTH * 3 + x * 3;

            gradient[index..index+3].copy_from_slice(&color);
        }
    }

    return ImageRead {
        pixels: res_pixels,
        min_temp: temp_min,
        max_temp: temp_max,
        gradient,
    }
}

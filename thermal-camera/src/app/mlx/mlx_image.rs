use super::{
    PIXEL_COUNT, GRADIENT_WIDTH, GRADIENT_HEIGHT, GRADIENT_COUNT,
    TemperatureRead, ImageRead, ColorTypes
};

// TODO: This function seems to be only half as bright as needed
// But not when calculating the gradient.
// Should be investigated
fn grayscale_function(temp: f32, min_temp: f32, max_temp: f32) -> [u8; 3] {
    let value: f32 = (temp - min_temp) * (255.0 / max_temp);
    let value_byte: u8 = value.round().max(0.0).min(255.0) as u8;

    return [value_byte; 3];
}

fn rgb_cheap_function(temp: f32, min_temp: f32, max_temp :f32) -> [u8; 3] {
    // Calculate interpolation value t, it is always between 0 and 1
    let mut t = (temp - min_temp) / (max_temp - min_temp);
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

fn rgb_hue_function(temp: f32, min_temp: f32, max_temp :f32) -> [u8; 3] {
    // Calculate interpolation value t, it is always between 0 and 1
    let mut t = (temp - min_temp) / (max_temp - min_temp);
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

pub fn color_image(color_type: ColorTypes, temperatures: TemperatureRead) -> ImageRead {
    let mut res_pixels: [u8; PIXEL_COUNT * 3] = [0x00; PIXEL_COUNT * 3];
    let min_temp = temperatures.min_temp;
    let max_temp = temperatures.max_temp;

    for i in 0..PIXEL_COUNT {
        let index = i * 3;
        let temp = temperatures.temperature_grid[i];

        let color = match color_type {
            ColorTypes::Gray => grayscale_function(temp, min_temp, max_temp),
            ColorTypes::Cheap => rgb_cheap_function(temp, min_temp, max_temp),
            ColorTypes::Hue => rgb_hue_function(temp, min_temp, max_temp),
        };

        res_pixels[index..index+3].copy_from_slice(&color);
    }

    return ImageRead {
        pixels: res_pixels,
        temperature_grid: temperatures.temperature_grid,
        min_temp,
        max_temp,
    }
}

pub fn color_gradient(color_type: ColorTypes) -> [u8; GRADIENT_COUNT * 3] {
    let mut gradient: [u8; GRADIENT_COUNT * 3] = [0x00; GRADIENT_COUNT * 3];
    for y in 0..GRADIENT_HEIGHT {
        let t = 1.0 - y as f32 / GRADIENT_HEIGHT as f32;
        let color = match color_type {
            ColorTypes::Gray => grayscale_function(t, 0.0, 1.0),
            ColorTypes::Cheap => rgb_cheap_function(t, 0.0, 1.0),
            ColorTypes::Hue => rgb_hue_function(t, 0.0, 1.0),
        };

        for x in 0..GRADIENT_WIDTH {
            let index = y * GRADIENT_WIDTH * 3 + x * 3;

            gradient[index..index+3].copy_from_slice(&color);
        }
    }

    return gradient;
}

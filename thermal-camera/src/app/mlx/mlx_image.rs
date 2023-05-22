use super::PIXEL_COUNT;

pub fn grayscale(temperatures: [f32; PIXEL_COUNT], temp_min: f32, temp_max: f32) -> [u8; PIXEL_COUNT] {
    let mut res: [u8; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

    for i in 0..PIXEL_COUNT {
        let value: f32 = (temperatures[i] - temp_min) * (255.0 / temp_max);
        res[i] = value.round().max(0.0).min(255.0) as u8;
    }

    return res;
}

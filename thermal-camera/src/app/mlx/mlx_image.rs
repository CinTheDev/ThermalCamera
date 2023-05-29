use super::PIXEL_COUNT;

pub fn grayscale(temperatures: [f32; PIXEL_COUNT], temp_min: f32, temp_max: f32) -> [u8; PIXEL_COUNT] {
    let mut res: [u8; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

    for i in 0..PIXEL_COUNT {
        let value: f32 = (temperatures[i] - temp_min) * (255.0 / temp_max);
        res[i] = value.round().max(0.0).min(255.0) as u8;
    }

    return res;
}

pub fn rgb(temperatures: [f32; PIXEL_COUNT], temp_min: f32, temp_max: f32) -> [u8; PIXEL_COUNT * 3] {
    let mut res: [u8; PIXEL_COUNT * 3] = [0x00; PIXEL_COUNT * 3];

    for i in 0..PIXEL_COUNT {
        let index = i * 3;

        // Calculate interpolation value t, it is always between 0 and 1
        let mut t = (temperatures[i] - temp_min) / (temp_max - temp_min);
        t = t.max(0.0).min(1.0);

        // Use special formulas to get rgb colors, I created a really simple and cheap one here.
        // Usually the rgb colors will be between 0 and 1, but sometimes it overshoots
        // Later it's being clamped
        let factor = 2.0;

        let r = 0_f32.max(-factor * (1.0 - t) + 1.0);
        let b = 0_f32.max(-factor * t         + 1.0);

        let g = 1.0 - r - b;

        // Convert to byte
        res[index + 0] = (r * 255.0).round().max(0.0).min(255.0) as u8;
        res[index + 1] = (g * 255.0).round().max(0.0).min(255.0) as u8;
        res[index + 2] = (b * 255.0).round().max(0.0).min(255.0) as u8;
    }

    return res;
}

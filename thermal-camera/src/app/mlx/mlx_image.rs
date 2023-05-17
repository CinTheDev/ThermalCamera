use super::PIXEL_COUNT;

pub fn grayscale_test(temperatures: &[f32]) -> [u8; PIXEL_COUNT] {
    // Let 20°C be black, and 40°C be white
    let mut res: [u8; PIXEL_COUNT] = [0x00; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        res[i] = (((temperatures[i] - 20.0) * (255.0/40.0)).round() as u8).max(0).min(255);
    }
    return res;
}

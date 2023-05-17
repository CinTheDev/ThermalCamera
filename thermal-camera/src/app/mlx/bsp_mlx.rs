use std::io::Write;
use std::fs::File;
use rppal::i2c::I2c;

use super::PIXEL_COUNT;

mod mlx_eeprom;

const CAM_ADDR: u8 = 0x33;

pub fn init() {
    mlx_eeprom::restore();
}

pub fn write(address: u16, data: u16) {
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(CAM_ADDR as u16).unwrap();

    let mut buffer: [u8; 4] = [0x00; 4];
    buffer[0..2].copy_from_slice(&address.to_be_bytes());
    buffer[2..4].copy_from_slice(&data.to_be_bytes());
    
    i2c.write(&buffer).expect("I2C write failed.");
}

pub fn read(address: u16, read_buffer: &mut [u8]) {
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(CAM_ADDR as u16).unwrap();

    let mut write_buffer: [u8; 2] = [0x00; 2];
    write_buffer.copy_from_slice(&address.to_be_bytes());

    i2c.write_read(&write_buffer, read_buffer).expect("I2C read failed.");
}

pub fn read_value(address: u16) -> u16 {
    let mut read_buffer: [u8; 2] = [0x00; 2];
    read(address, &mut read_buffer);

    return u16::from_be_bytes(read_buffer);
}

/*
pub fn write_image(path: &str, img: &[u8], width: usize, height: usize) {
    // Raw image is graymap
    let mut file = File::create(path).unwrap();

    let err_msg = "Failed to write image to disk.";

    // Write header info
    file.write(b"P5\n").expect(err_msg);
    file.write(format!("{} {}\n", width, height).as_bytes()).expect(err_msg);
    file.write(b"255\n").expect(err_msg);

    // Write image contents in binary format
    file.write(img).expect(err_msg);
}
*/

pub fn evaluate_image(pix_data: [u16; PIXEL_COUNT]) -> [f32; PIXEL_COUNT] {
    return mlx_eeprom::evaluate(pix_data);
}

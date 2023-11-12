use rppal::i2c::I2c;
use std::sync::Mutex;
use lazy_static::lazy_static;

mod mlx_eeprom;

use super::PIXEL_COUNT;

const CAM_ADDR: u8 = 0x33;

lazy_static! {
    static ref I2C_MUTEX: Mutex<u32> = Mutex::new(0);
}

pub fn init() {
    mlx_eeprom::restore();
}

pub fn write(address: u16, data: u16) -> Result<(), String> {
    let _i2c_lock = I2C_MUTEX.lock().unwrap();

    let i2c_p_response = I2c::new();

    if i2c_p_response.is_err() {
        return Err("Error when accessing i2c peripheral for write".to_string());
    }

    let mut i2c = i2c_p_response.unwrap();
    i2c.set_slave_address(CAM_ADDR as u16).unwrap();

    let mut buffer: [u8; 4] = [0x00; 4];
    buffer[0..2].copy_from_slice(&address.to_be_bytes());
    buffer[2..4].copy_from_slice(&data.to_be_bytes());
    
    let i2c_write_response = i2c.write(&buffer);
    if i2c_write_response.is_err() {
        return Err("Error in i2c protocol: write".to_string());
    }

    return Ok(());
}

pub fn read(address: u16, read_buffer: &mut [u8]) -> Result<(), String> {
    let _i2c_lock = I2C_MUTEX.lock().unwrap();

    let i2c_p_response = I2c::new();

    if i2c_p_response.is_err() {
        return Err("Error when accessing i2c peripheral for read".to_string());
    }

    let mut i2c = i2c_p_response.unwrap();
    i2c.set_slave_address(CAM_ADDR as u16).unwrap();

    let mut write_buffer: [u8; 2] = [0x00; 2];
    write_buffer.copy_from_slice(&address.to_be_bytes());

    let i2c_read_response = i2c.write_read(&write_buffer, read_buffer);
    if i2c_read_response.is_err() {
        return Err("Error in i2c protocol: read".to_string());
    }

    return Ok(());
}

pub fn read_value(address: u16) -> u16 {
    let mut read_buffer: [u8; 2] = [0x00; 2];
    read(address, &mut read_buffer);

    return u16::from_be_bytes(read_buffer);
}

pub fn evaluate_image(pix_data: [u16; PIXEL_COUNT]) -> [f32; PIXEL_COUNT] {
    return mlx_eeprom::evaluate(pix_data);
}

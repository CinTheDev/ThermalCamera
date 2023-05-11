use std::io::Write;
use std::fs::File;
use rppal::i2c::I2c;

pub const PIXELS_WIDTH: usize = 32;
pub const PIXELS_HEIGHT: usize = 24;
pub const PIXEL_COUNT: usize = PIXELS_WIDTH * PIXELS_HEIGHT;

struct eeprom_raw {
    // VDD

    // Ta

    // Offset

    // Sensitivity a (i, j)

    // Kv (i, j)

    // Kta (i, j)

    // GAIN

    // KsTa

    // Corner temperatures

    // KsTo

    // Ranged sensitivity correction

    // Sensitivity a_CP

    // Offset of CP

    // Kv CP

    // Kta CP

    // TGC

    // Resolution control
}

struct eeprom_vars {
    K_Vdd: i32,
    VDD_25: i32,

    T_a: i32,

    pix_os_ref: [i32; PIXEL_COUNT],

    a: [i32; PIXEL_COUNT],

    Kv: [i32; PIXEL_COUNT],

    Kta: [i32; PIXEL_COUNT],

    GAIN: i32,

    KsTa: i32,

    Step: i32,
    CT3: i32,
    CT4: i32,

    Ks_To1: i32,
    Ks_To2: i32,
    Ks_To3: i32,
    Ks_To4: i32,

    Alpha_corr_1: i32,
    Alpha_corr_2: i32,
    Alpha_corr_3: i32,
    Alpha_corr_4: i32,

    a_cp_0: i32,
    a_cp_1: i32,

    off_cp_0: i32,
    off_cp_1: i32,

    Kv_cp: i32,

    K_Ta_cp: i32,

    TGC: i32,

    Resolution: u32,
}

const CAM_ADDR: u8 = 0x33;

pub fn write(address: u16, data: u16) {
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(CAM_ADDR as u16).unwrap();

    let mut buffer: [u8; 4] = [0x00; 4];
    buffer[0..2].copy_from_slice(&address.to_be_bytes());
    buffer[2..4].copy_from_slice(&data.to_be_bytes());
    
    i2c.write(&buffer).expect("I2C write failed.");
}

pub fn read(address: u16) -> u16 {
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(CAM_ADDR as u16).unwrap();

    let mut write_buffer: [u8; 2] = [0x00; 2];
    write_buffer.copy_from_slice(&address.to_be_bytes());

    let mut read_buffer: [u8; 2] = [0x00; 2];

    i2c.write_read(&write_buffer, &mut read_buffer).expect("I2C read failed.");

    let output = u16::from_be_bytes(read_buffer);
    return output;
}

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

pub fn restore() {
    // VDD

    // Ta

    // Offset

    // Sensitivity a (i, j)

    // Kv (i, j)

    // Kta (i, j)

    // GAIN

    // KsTa

    // Corner temperatures

    // KsTo

    // Ranged sensitivity correction

    // Sensitivity a_CP

    // Offset of CP

    // Kv CP

    // Kta CP

    // TGC

    // Resolution control
}

pub fn calibrate() {
    // Calculate Voltage

    // Calculate Ambient temperature

    // Compensate for gain

    // Offset, VDD and Ta

    // Emissivity compensation

    // Gradient compensation

    // Normalize to sensitivity

    // Calculate To
}

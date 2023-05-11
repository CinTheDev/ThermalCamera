use std::io::Write;
use std::fs::File;
use rppal::i2c::I2c;
use lazy_static::lazy_static;
use power_of_two::power_of_two;

pub const PIXELS_WIDTH: usize = 32;
pub const PIXELS_HEIGHT: usize = 24;
pub const PIXEL_COUNT: usize = PIXELS_WIDTH * PIXELS_HEIGHT;

const EEPROM_SIZE: usize = 767;

static mut EEPROM_RAW: [i16; EEPROM_SIZE] = [0x00; EEPROM_SIZE];

fn read_eeprom() {
    let mut d: [u8; EEPROM_SIZE * 2] = [0x00; EEPROM_SIZE * 2];
    read(0x2440, &mut d);

    let mut converted: [i16; EEPROM_SIZE] = [0x00; EEPROM_SIZE];

    for i in 0..EEPROM_SIZE {
        let msb: i16 = d[i * 2 + 0] as i16;
        let lsb: i16 = d[i * 2 + 1] as i16;
        converted[i] = (msb << 8) | lsb;
    }

    unsafe { EEPROM_RAW = converted };
}

fn get_eeprom_val(address: u16) -> i16 {
    let index:usize = (address - 0x2440) as usize;
    return unsafe { EEPROM_RAW[index] };
}

struct eeprom_vars {
    K_Vdd: i16,
    VDD_25: i16,

    T_a: i16,

    pix_os_ref: [i16; PIXEL_COUNT],

    a: [i16; PIXEL_COUNT],

    Kv: [i16; PIXEL_COUNT],

    Kta: [i16; PIXEL_COUNT],

    GAIN: i16,

    KsTa: i16,

    Step: i16,
    CT3: i16,
    CT4: i16,

    Ks_To1: i16,
    Ks_To2: i16,
    Ks_To3: i16,
    Ks_To4: i16,

    Alpha_corr_1: i16,
    Alpha_corr_2: i16,
    Alpha_corr_3: i16,
    Alpha_corr_4: i16,

    a_cp_0: i16,
    a_cp_1: i16,

    off_cp_0: i16,
    off_cp_1: i16,

    Kv_cp: i16,

    K_Ta_cp: i16,

    TGC: i16,

    Resolution: u16,
}

lazy_static! {
    static ref EEPROM_VARS: eeprom_vars = restore();
}

const CAM_ADDR: u8 = 0x33;

pub fn init() {
    restore();
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

fn restore() -> eeprom_vars {
    // Read eeprom data
    read_eeprom();

    // VDD
    let mut K_Vdd: i16 = (get_eeprom_val(0x2433) & 0xFF00) / power_of_two!(8) as i16;
    if K_Vdd > 127 {
        K_Vdd = K_Vdd - 256;
    }
    let mut VDD: i16 = get_eeprom_val(0x2433) & 0x00FF;
    VDD = (VDD - 256) * power_of_two!(5) as i16 - power_of_two!(13) as i16;

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

    return eeprom_vars {
        K_Vdd: K_Vdd,
        VDD_25: VDD,
    }
}

fn calibrate() {
    // Calculate Voltage

    // Calculate Ambient temperature

    // Compensate for gain

    // Offset, VDD and Ta

    // Emissivity compensation

    // Gradient compensation

    // Normalize to sensitivity

    // Calculate To
}

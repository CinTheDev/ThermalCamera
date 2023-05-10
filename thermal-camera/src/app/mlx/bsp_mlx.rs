use std::io::Write;
use std::fs::File;
use rppal::i2c::I2c;

const CAM_ADDR: u8 = 0x33;

pub fn write(address: u16, data: u16) {
    /*
    let mut command = Command::new("i2cset");
    command.arg("-y");
    command.arg(CAM_ADDR.to_string());
    command.arg("1");

    command.arg(((address & 0xFF00) >> 8).to_string());
    command.arg(((address & 0x00FF) >> 0).to_string());

    command.arg(((data & 0xFF00) >> 8).to_string());
    command.arg(((data & 0x00FF) >> 0).to_string());

    command.output().expect("I2C write failed.");
    */

    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(CAM_ADDR as u16).unwrap();

    let mut buffer: [u8; 4] = [0x00; 4];
    buffer[0..2].copy_from_slice(&address.to_le_bytes());
    buffer[2..4].copy_from_slice(&data.to_le_bytes());

    i2c.write(&buffer).expect("I2C write failed.");
}

pub fn read(address: u16) -> u16 {
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(CAM_ADDR as u16).unwrap();

    let mut write_buffer: [u8; 2] = [0x00; 2];
    write_buffer.copy_from_slice(&address.to_le_bytes());

    let mut read_buffer: [u8; 2] = [0x00; 2];

    i2c.write_read(&write_buffer, &mut read_buffer).expect("I2C read failed.");

    let output = u16::from_le_bytes(read_buffer);
    return output;
    /*
    let mut command = Command::new("i2ctransfer");
    command.arg("-y");
    command.arg("1");

    // Set pointer address
    let com_write = format!("{}{}", "w2@", CAM_ADDR.to_string());
    command.arg(com_write);

    let addr1 = (address & 0xFF00) >> 8;
    let addr2 = (address & 0x00FF) >> 0;
    command.arg(addr1.to_string());
    command.arg(addr2.to_string());

    // Receive value command
    let com_read = format!("{}{}", "r2@", CAM_ADDR.to_string());
    command.arg(com_read);

    // Get output and format into u16
    command.stdout(Stdio::piped());
    let out = command.output().expect("I2C read failed.");
    let output_string = String::from_utf8_lossy(&out.stdout);

    //print!("Control: {}", output_string);

    let values: Vec<&str> = output_string.split_ascii_whitespace().collect();
    let mut res: u16 = 0;

    for i in 0..2 {
        let without_prefix = values[i].trim_start_matches("0x");
        let val = u16::from_str_radix(without_prefix, 16).unwrap();
        res |= val << ((1 - i) * 8);
    }

    //println!("Value: {:x}", res);

    return res;
    */
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

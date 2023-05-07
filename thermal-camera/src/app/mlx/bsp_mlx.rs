use std::process::{Command, Stdio};

const CAM_ADDR: u8 = 0x33;

pub fn write(data: &[u8]) {
    let mut command = Command::new("i2cset");
    command.arg("-y");
    command.arg(CAM_ADDR.to_string());
    command.arg("1");

    for d in data {
        command.arg(d.to_string());
    }

    command.output().expect("I2C write failed.");
}

pub fn read(address: u16) -> u16 {
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

    // Receive value
    let com_read = format!("{}{}", "r2@", CAM_ADDR.to_string());
    command.arg(com_read);

    // TODO
    command.stdout(Stdio::piped());
    let out = command.output().expect("I2C read failed.");
    print!("{}", String::from_utf8_lossy(&out.stdout));

    return 0;
}

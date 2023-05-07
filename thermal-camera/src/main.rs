use std::{process::Command, os::unix::process::CommandExt};

fn main() {
    println!("Hello, world!");

    let mut command = Command::new("echo");
    command.arg("Hi");
    command.exec();
}

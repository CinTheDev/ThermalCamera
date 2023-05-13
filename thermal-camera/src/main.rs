use std::env;

mod app;

fn main() {
    println!("Thermal Camera reading.");
    env::set_var("RUST_BACKTRACE", "1");

    app::init();
}

use structopt::StructOpt;

mod bsp;
mod mlx;

pub fn init() {
    mlx::init();
    let opt = Opt::from_args();
    println!("Filename: {}", opt.filename);
}

#[derive(Debug, StructOpt)]
#[structopt(name = "MLX driver")]
struct Opt {
    #[structopt()]
    filename: String,
}

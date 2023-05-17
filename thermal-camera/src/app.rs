use structopt::StructOpt;

mod bsp;
mod mlx;

pub fn init() {
    mlx::init();

    let opt = Opt::from_args();

    mlx::grayscale(opt.filename.as_str(), 20.0, 40.0);
}

#[derive(Debug, StructOpt)]
#[structopt(name = "MLX driver")]
struct Opt {
    #[structopt()]
    filename: String,
}

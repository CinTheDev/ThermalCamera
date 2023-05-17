use structopt::StructOpt;

mod bsp;
mod mlx;

pub fn init() {
    mlx::init();

    let opt = Opt::from_args();

    mlx::grayscale(opt.filename.as_str(), opt.min, opt.max);
}

#[derive(Debug, StructOpt)]
#[structopt(name = "MLX driver")]
struct Opt {
    #[structopt()]
    filename: String,

    #[structopt(long, default_value = "20")]
    min: f32,

    #[structopt(long, default_value = "40")]
    max: f32,
}

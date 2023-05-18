use structopt::StructOpt;

mod bsp;
mod mlx;

pub fn run() {
    mlx::init();

    let opt = Opt::from_args();

    let filename = opt.filename.as_str();
    let min = opt.min;
    let max = opt.max;

    mlx::grayscale(filename, min, max);
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

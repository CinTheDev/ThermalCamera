use structopt::StructOpt;

mod bsp;
mod mlx;

// TODO: use this instead of strings
enum ColorTypes {
    Gray,
    Cheap,
}

pub fn run() {
    mlx::init();

    let opt = Opt::from_args();

    let filename = opt.filename.as_str();
    let min = opt.min;
    let max = opt.max;

    let width = mlx::PIXELS_WIDTH;
    let height = mlx::PIXELS_HEIGHT;
    let output = mlx::grayscale(min, max);

    bsp::write_grayscale(filename, &output, width, height);
}

#[derive(Debug, StructOpt)]
#[structopt(name = "MLX driver")]
struct Opt {
    #[structopt(default_value = "out.png")]
    filename: String,

    #[structopt(default_value = "gray")]
    color_type: String,

    #[structopt(long, default_value = "20")]
    min: f32,

    #[structopt(long, default_value = "40")]
    max: f32,
}

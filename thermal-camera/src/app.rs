use std::str::FromStr;
use structopt::StructOpt;

mod bsp;
mod mlx;

pub fn run() {
    mlx::init();

    let opt = Opt::from_args();

    let filename = opt.filename.as_str();
    let min = opt.min;
    let max = opt.max;
    let col = opt.color_type;

    let width = mlx::PIXELS_WIDTH;
    let height = mlx::PIXELS_HEIGHT;

    let output = get_mlx_output(col, min, max);

    bsp::write_rgb(filename, &output, width, height);
}

fn get_mlx_output(color_type: String, temp_min: f32, temp_max: f32) -> [u8; mlx::PIXEL_COUNT * 3] {
    match color_type.as_str() {
        "gray" => return mlx::grayscale(temp_min, temp_max),
        "cheap" => return mlx::colored_cheap(temp_min, temp_max),

        _ => panic!()
    }
}

// TODO: use this instead of strings
enum ColorTypes {
    Gray,
    Cheap,
}

impl FromStr for ColorTypes {
    type Err = &'static str;
    fn from_str(color_type: &str) -> Result<Self, Self::Err> {
        match color_type {
            "gray" => Ok(ColorTypes::Gray),
            "cheap" => Ok(ColorTypes::Cheap),

            _ => Err("Unrecognised color type"),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "MLX driver")]
struct Opt {
    #[structopt(default_value = "out.png")]
    filename: String,

    #[structopt(default_value = "cheap")]
    color_type: String,

    #[structopt(long, default_value = "20")]
    min: f32,

    #[structopt(long, default_value = "40")]
    max: f32,
}

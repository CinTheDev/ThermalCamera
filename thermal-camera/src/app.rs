use std::str::FromStr;
use structopt::StructOpt;

mod bsp;
mod mlx;
mod window;

use mlx::ColorTypes;
use mlx::ImageRead;

pub fn run() {
    mlx::init();

    let opt = Opt::from_args();

    if opt.windowed {
        window::open_window(opt);
    }
    else {
        let filename = opt.filename.as_str();
        let width = mlx::PIXELS_WIDTH;
        let height = mlx::PIXELS_HEIGHT;

        let output = get_mlx_output(&opt);

        bsp::write_rgb(filename, &output.pixels, width, height);
    }
}

fn get_mlx_output(args: &Opt) -> ImageRead {
    return mlx::take_image(&args.color_type);
}

impl FromStr for ColorTypes {
    type Err = &'static str;
    
    fn from_str(color_type: &str) -> Result<Self, Self::Err> {
        match color_type {
            "gray" => Ok(ColorTypes::Gray),
            "cheap" => Ok(ColorTypes::Cheap),
            "hue" => Ok(ColorTypes::Hue),

            _ => Err("Unrecognised color type"),
        }
    }
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "MLX driver")]
pub struct Opt {
    #[structopt(default_value = "out.png")]
    filename: String,

    #[structopt(default_value = "hue")]
    color_type: ColorTypes,

    #[structopt(short, long)]
    windowed: bool,
}

impl Default for Opt {
    fn default() -> Self {
        Self {
            filename: "out.png".to_string(),
            color_type: ColorTypes::Cheap,
            windowed: false,
        }
    }
}

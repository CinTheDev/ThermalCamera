use std::str::FromStr;
use structopt::StructOpt;

mod bsp;
mod mlx;
mod window;

use mlx::ColorTypes;
use mlx::ImageRead;

pub fn run() {
    let opt = Opt::from_args();

    if opt.windowed {
        window::open_window(opt);
    }
    else {
        // TODO: Update this
        let path = opt.filename.as_str();
        let width = mlx::PIXELS_WIDTH as u32;
        let height = mlx::PIXELS_HEIGHT as u32;

        let output = get_mlx_output(&opt);

        bsp::write_png(path, &output.pixels, width, height);
    }
}

fn get_mlx_output(args: &Opt) -> ImageRead {
    return mlx::take_image(&args.color_type).unwrap();
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

    #[structopt(default_value = "2")]
    framerate: mlx::Framerates,

    #[structopt(short, long)]
    windowed: bool,

    #[structopt(short, long)]
    left_handed: bool,
}

impl Default for Opt {
    fn default() -> Self {
        Self {
            filename: "out.png".to_string(),
            color_type: ColorTypes::Cheap,
            framerate: mlx::Framerates::Two,
            windowed: false,
            left_handed: false,
        }
    }
}

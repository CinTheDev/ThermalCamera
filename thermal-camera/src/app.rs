use std::str::FromStr;
use structopt::StructOpt;

mod bsp;
mod mlx;
mod window;

use mlx::ColorTypes;

pub fn run() {
    mlx::init();
    bsp::usb_test();

    let opt = Opt::from_args();

    if opt.windowed {
        window::open_window(opt);
    }
    else {
        let filename = opt.filename.as_str();
        let col = opt.color_type;
        let min = opt.min;
        let max = opt.max;
        let width = mlx::PIXELS_WIDTH;
        let height = mlx::PIXELS_HEIGHT;

        let output = get_mlx_output(col, min, max);

        bsp::write_rgb(filename, &output, width, height);
    }
}

fn get_mlx_output(color_type: ColorTypes, temp_min: f32, temp_max: f32) -> [u8; mlx::PIXEL_COUNT * 3] {
    match color_type {
        ColorTypes::Gray => return mlx::grayscale(temp_min, temp_max),
        ColorTypes::Cheap => return mlx::colored_cheap(temp_min, temp_max),
    }
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

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "MLX driver")]
pub struct Opt {
    #[structopt(default_value = "out.png")]
    filename: String,

    #[structopt(default_value = "cheap")]
    color_type: ColorTypes,

    #[structopt(short, long)]
    windowed: bool,

    #[structopt(long, default_value = "20")]
    min: f32,

    #[structopt(long, default_value = "40")]
    max: f32,
}

impl Default for Opt {
    fn default() -> Self {
        Self {
            filename: "out.png".to_string(),
            color_type: ColorTypes::Cheap,
            windowed: false,
            min: 20.0,
            max: 40.0,
        }
    }
}

use eframe::egui;
use super::mlx;
use mlx::ImageRead;

use super::bsp;
use std::thread;
use std::sync::mpsc::{self, Receiver, Sender};

pub use super::Opt;

mod display;
mod controls;
mod options;

// How much of the screen is covered by these widgets
const SCALE_X_SPACE: f32 = 0.1;
const CONTROLS_X_SPACE: f32 = 0.15;

// Fill rest of space
const IMAGE_X_SPACE: f32 = 1.0 - SCALE_X_SPACE - CONTROLS_X_SPACE;

pub fn open_window(args: Opt) {
    let native_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };

    eframe::run_native(
        "Thermal Camera",
        native_options,
        Box::new(|cc| Box::new(ThermalApp::new(cc, args))),
    )
    .unwrap();
}

#[derive(Default)]
pub struct ThermalApp {
    window_size: egui::Vec2,

    options: Opt,

    last_read: Option<mlx::ImageRead>,

    picture: Option<egui::TextureHandle>,
    picture_options: egui::TextureOptions,

    raw_scale: Option<[u8; mlx::GRADIENT_COUNT * 3]>,
    scale: Option<egui::TextureHandle>,
    scale_bound: (f32, f32),

    show_options: bool,

    image_rx: Option<mpsc::Receiver<ImageRead>>,
    rx_active: bool,
    args_tx: Option<mpsc::Sender<Opt>>,

    usb_detected: bool,
}

impl ThermalApp {
    fn new(_cc: &eframe::CreationContext<'_>, args: Opt) -> Self {
        let mut s = Self {
            options: args,
            rx_active: true,
            picture_options: egui::TextureOptions::NEAREST,
            scale_bound: (20.0, 40.0),
            ..Default::default()
        };

        display::image::init_image_texture(&mut s, &_cc.egui_ctx);
        display::scale::init_scale(&mut s, &_cc.egui_ctx);
        display::scale::update_scale(&mut s);

        return s;
    }

    fn get_thread_receiver(&mut self, ctx: &egui::Context) -> &mut Receiver<ImageRead> {
        let options_clone = self.options.clone();

        self.image_rx.get_or_insert_with(|| {
            let (tx, rx) = mpsc::channel();
            let ctx_clone = ctx.clone();

            let (args_tx, args_rx) = mpsc::channel();
            args_tx.send(options_clone).unwrap();
            self.args_tx = Some(args_tx);

            thread::spawn(|| ThermalApp::continuuos_read(args_rx, ctx_clone, tx));

            return rx;
        })
    }

    fn continuuos_read(args_rx: Receiver<Opt>, ctx: egui::Context, tx: Sender<ImageRead>) -> ! {
        let mut args: Option<Opt> = Some(Opt::default());
        loop {
            let temp_grid = mlx::read_temperatures();

            let r = args_rx.try_recv();
            if r.is_ok() {
                args.replace(r.unwrap());
            }

            let color_grid = mlx::mlx_image::color_image(&args.as_ref().unwrap().color_type, &temp_grid);
            tx.send(color_grid).unwrap();
            ctx.request_repaint();
        }
    }

    fn recolor_image(&mut self, ctx: &egui::Context) {
        if self.last_read.is_none() { return; }

        let last_read = self.last_read.as_ref().unwrap();

        let color_grid = mlx::mlx_image::color_image(&self.options.color_type, &last_read.temperature_read);
        
        let img = egui::ColorImage::from_rgb(
            [mlx::PIXELS_WIDTH, mlx::PIXELS_HEIGHT],
            &color_grid.pixels
        );

        self.picture.as_mut().unwrap().set(img, self.picture_options);

        self.last_read.replace(color_grid);

        ctx.request_repaint();
    }

    fn update_options(&mut self) {
        if self.args_tx.is_none() { return; }
        let tx: &Sender<Opt> = self.args_tx.as_ref().unwrap();
        tx.send(self.options.clone()).unwrap();

        display::scale::update_scale(self);
    }

    fn save_image(&mut self) {
        if self.picture.is_none() { return }
        if !bsp::check_usb() { return }

        let raw_img = self.last_read.as_ref().unwrap().pixels;

        let path = bsp::get_usb_path();

        bsp::write_png(
            &path,
            &raw_img,
            mlx::PIXELS_WIDTH as u32,
            mlx::PIXELS_HEIGHT as u32,
        );
    }

    fn check_usb(&mut self) {
        self.usb_detected = bsp::check_usb();
    }
}

impl eframe::App for ThermalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_usb();

        egui::CentralPanel::default().show(ctx, |ui| {
            self.window_size = ui.available_size();

            ui.horizontal_centered(|ui| {
                display::show(self, ui, ctx);    

                controls::show(self, ui);
            });

            options::show(self, ui);
        });
    }
}

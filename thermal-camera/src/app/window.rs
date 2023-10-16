use eframe::egui;
use super::mlx;
use mlx::ImageRead;

use super::bsp;
use std::thread;
use std::sync::mpsc;

pub use super::Opt;

mod display;

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
    options: Opt,

    temperature_grid: Option<[f32; mlx::PIXEL_COUNT]>,

    raw_picture: Option<[u8; mlx::PIXEL_COUNT * 3]>,
    picture: Option<egui::TextureHandle>,
    picture_options: egui::TextureOptions,

    raw_scale: Option<[u8; mlx::GRADIENT_COUNT * 3]>,
    scale: Option<egui::TextureHandle>,
    scale_bound: (f32, f32),

    image_rx: Option<mpsc::Receiver<ImageRead>>,
    rx_active: bool,

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

        s.update_scale(s.options.color_type);

        return s;
    }

    fn get_thread_receiver(&mut self, ctx: &egui::Context) -> &mut mpsc::Receiver<ImageRead> {
        self.image_rx.get_or_insert_with(|| {
            let (tx, rx) = mpsc::channel();
            let ctx_clone = ctx.clone();
            let args_clone = self.options.clone();

            thread::spawn(move || ThermalApp::continuuos_read(args_clone, ctx_clone, tx));

            return rx;
        })
    }

    fn continuuos_read(args: Opt, ctx: egui::Context, tx: mpsc::Sender<ImageRead>) -> ! {
        loop {
            let img = mlx::take_image(&args);
            tx.send(img).unwrap();
            ctx.request_repaint();
        }
    }

    fn save_image(&mut self) {
        if self.picture.is_none() { return }
        if !bsp::check_usb() { return }

        let raw_img = self.raw_picture.as_ref().unwrap();

        let path = bsp::get_usb_path("png".to_string());

        bsp::write_rgb(
            &path,
            raw_img,
            mlx::PIXELS_WIDTH,
            mlx::PIXELS_HEIGHT,
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
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    if ui.button("Freeze image").clicked() {
                        self.rx_active = !self.rx_active;
                    }

                    let save_button = ui.add_enabled(
                        self.usb_detected,
                        egui::Button::new("Save image")
                    );
                    if save_button.clicked() {
                        self.save_image();
                        println!("Image saved");
                    }
            });

            display::show(self, ui, ctx);    
        });
    }
}

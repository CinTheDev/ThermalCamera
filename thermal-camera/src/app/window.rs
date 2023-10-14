use eframe::egui;
use super::mlx;
use mlx::ImageRead;

use super::bsp;
use std::thread;
use std::sync::mpsc;

pub use super::Opt;

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
struct ThermalApp {
    options: Opt,

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

    fn show_image(&mut self, ui: &mut egui::Ui) {
        let texture: &egui::TextureHandle = self.picture.get_or_insert_with(|| {
            let raw_img = self.raw_picture.get_or_insert_with(|| {
                [0x00; mlx::PIXEL_COUNT * 3]
            });

            let img = egui::ColorImage::from_rgb(
                [mlx::PIXELS_WIDTH, mlx::PIXELS_HEIGHT],
                raw_img
            );

            ui.ctx()
                .load_texture("Picture", img, self.picture_options)
        });

        let height = ui.available_height();
        let width = height * (mlx::PIXELS_WIDTH as f32 / mlx::PIXELS_HEIGHT as f32);
        let size = egui::Vec2 {x: width, y: height};

        ui.image(texture, size);
    }

    fn show_scale(&mut self, ui: &mut egui::Ui) {
        let texture: &egui::TextureHandle = self.scale.get_or_insert_with(|| {
            let raw_scale = self.raw_scale.get_or_insert_with(|| {
                [0x00; mlx::GRADIENT_COUNT * 3]
            });

            let img = egui::ColorImage::from_rgb(
                [mlx::GRADIENT_WIDTH, mlx::GRADIENT_HEIGHT],
                raw_scale
            );

            ui.ctx().load_texture("Scale", img, self.picture_options)
        });

        ui.vertical(
            |ui| {
                let string_min_temp = format!("{:.1}", self.scale_bound.0);
                let string_max_temp = format!("{:.1}", self.scale_bound.1);
                ui.label(string_max_temp);

                let height = ui.available_height() - 20.0;
                let width = height * (mlx::GRADIENT_WIDTH as f32 / mlx::GRADIENT_HEIGHT as f32);
                let size = egui::Vec2 {x: width, y: height};
                ui.image(texture, size);

                ui.label(string_min_temp);
            }
        );
    }
    
    fn update_image(&mut self, ctx: &egui::Context) {
        // Don't update image if not supposed to
        if !self.rx_active { return }

        let rx = self.get_thread_receiver(ctx);

        let rx_img = rx.try_recv();

        if rx_img.is_ok() {
            let raw_img = rx_img.unwrap();
            self.raw_picture.replace(raw_img.pixels);

            let img = egui::ColorImage::from_rgb(
                [mlx::PIXELS_WIDTH, mlx::PIXELS_HEIGHT],
                &raw_img.pixels
            );

            self.picture.as_mut().unwrap().set(img, self.picture_options);

            self.scale_bound = (raw_img.min_temp, raw_img.max_temp);
        }
    }

    fn update_scale(&mut self, color_type: mlx::ColorTypes) {
        let gradient = mlx::get_scale(color_type);

        self.raw_scale.replace(gradient);
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

            self.update_image(ctx);

            ui.vertical_centered(
                |ui| {
                    ui.horizontal_centered(
                        |ui| {
                            self.show_scale(ui);
                            self.show_image(ui);
                        }
                    );
            });
        });
    }
}

use eframe::egui;
use super::mlx;
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
    image_rx: Option<mpsc::Receiver<[u8; mlx::PIXEL_COUNT * 3]>>,
    rx_active: bool,
}

impl ThermalApp {
    fn new(_cc: &eframe::CreationContext<'_>, args: Opt) -> Self {
        Self {
            options: args,
            rx_active: true,
            picture_options: egui::TextureOptions::NEAREST,
            ..Default::default()
        }
    }

    fn get_thread_receiver(&mut self, ctx: &egui::Context) -> &mut mpsc::Receiver<[u8; mlx::PIXEL_COUNT * 3]> {
        self.image_rx.get_or_insert_with(|| {
            let (tx, rx) = mpsc::channel();
            let ctx_clone = ctx.clone();
            let args_clone = self.options.clone();

            thread::spawn(move || ThermalApp::continuuos_read(args_clone, ctx_clone, tx));

            return rx;
        })
    }

    fn continuuos_read(args: Opt, ctx: egui::Context, tx: mpsc::Sender<[u8; mlx::PIXEL_COUNT * 3]>) -> ! {
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

        let space = ui.available_rect_before_wrap();
        let aspect_ratio = space.width() / space.height();
        let desired_ratio = mlx::PIXELS_WIDTH as f32 / mlx::PIXELS_HEIGHT as f32;
        let new_rect;

        if aspect_ratio > desired_ratio {
            // Width must be smaller
            let factor = desired_ratio / aspect_ratio;
            let new_width = space.width() * factor;
            let diff = (space.width() - new_width) * 0.5;

            new_rect = space.shrink2(egui::Vec2 {x: diff, y: 0.0});
        }
        else {
            // Height must be smaller
            let factor = aspect_ratio / desired_ratio;
            let new_height = space.height() * factor;
            let diff = (space.height() - new_height) * 0.5;

            new_rect = space.shrink2(egui::Vec2 {x: 0.0, y: diff});
        }

        ui.painter().image(
            texture.id(),
            new_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE
        );
    }
    
    fn update_image(&mut self, ctx: &egui::Context) {
        // Don't update image if not supposed to
        if !self.rx_active { return }

        let rx = self.get_thread_receiver(ctx);

        let rx_img = rx.try_recv();

        if rx_img.is_ok() {
            let raw_img = rx_img.unwrap();
            self.raw_picture.replace(raw_img);

            let img = egui::ColorImage::from_rgb(
                [mlx::PIXELS_WIDTH, mlx::PIXELS_HEIGHT],
                &raw_img
            );

            self.picture.as_mut().unwrap().set(img, self.picture_options);
        }
    }

    fn save_image(&mut self) {
        if self.picture.is_none() { return }

        let raw_img = self.raw_picture.as_ref().unwrap();

        bsp::write_rgb(
            &self.options.filename,
            raw_img,
            mlx::PIXELS_WIDTH,
            mlx::PIXELS_HEIGHT,
        );
    }
}

impl eframe::App for ThermalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    if ui.button("Freeze image").clicked() {
                        self.rx_active = !self.rx_active;
                    }

                    if ui.button("Save image").clicked() {
                        self.save_image();
                        println!("Image saved");
                    }
            });

            self.update_image(ctx);

            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center)
                    .with_main_justify(true),
                |ui| {
                    self.show_image(ui);
            });
        });
    }
}

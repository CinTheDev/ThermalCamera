use eframe::egui;
pub use super::Opt;
use super::mlx;
use std::thread;

pub fn open_window(args: Opt) {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Thermal Camera",
        native_options,
        Box::new(|cc| Box::new(ThermalApp::new(cc, args))),
    )
    .unwrap();
}

struct ThermalApp {
    options: Opt,
    picture: Option<egui::TextureHandle>,
    mlx_thread: Option<thread::Thread>,
    image_ready: bool,
    raw_image: [u8; mlx::PIXEL_COUNT * 3],
}

impl Default for ThermalApp {
    fn default() -> Self {
        Self {
            options: Default::default(),
            picture: Default::default(),
            mlx_thread: Default::default(),
            image_ready: false,
            raw_image: [0x00; mlx::PIXEL_COUNT * 3],
        }
    }
}

impl ThermalApp {
    fn new(_cc: &eframe::CreationContext<'_>, args: Opt) -> Self {
        Self {
            options: args,
            ..Default::default()
        }
    }

    fn show_image(&mut self, ui: &mut egui::Ui) {
        let texture: &egui::TextureHandle = self.picture.get_or_insert_with(|| {
            let img = egui::ColorImage::from_rgb(
                [mlx::PIXELS_WIDTH, mlx::PIXELS_HEIGHT],
                &[0x00; mlx::PIXEL_COUNT * 3]
            );

            ui.ctx()
                .load_texture("Picture", img, Default::default())
        });

        ui.painter().image(
            texture.id(),
            ui.available_rect_before_wrap(),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE
        );
    }

    fn take_image(&mut self, ui: &mut egui::Ui) {
        let img = egui::ColorImage::from_rgb(
            [mlx::PIXELS_WIDTH, mlx::PIXELS_HEIGHT],
            &mlx::colored_cheap(self.options.min, self.options.max)
        );

        self.picture.replace(ui.ctx().load_texture("Picture", img, Default::default()));
    }
}

impl eframe::App for ThermalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: format this in a more sensical way
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                if ui.button("Test picture").clicked() {
                    self.take_image(ui);
                }
            });

            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center).with_main_justify(true), |ui| {
                self.show_image(ui);
            });
        });
    }
}

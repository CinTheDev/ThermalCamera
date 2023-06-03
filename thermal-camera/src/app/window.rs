use eframe::egui;
pub use super::Opt;
use super::mlx;

pub fn open_window(args: Opt) {
    let native_options = eframe::NativeOptions::default();
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
    picture: Option<egui::TextureHandle>,
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

        ui.image(texture, texture.size_vec2());
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
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                if ui.button("Test picture").clicked() {
                    self.take_image(ui);
                }
    
                self.show_image(ui);
            })
        });
    }
}

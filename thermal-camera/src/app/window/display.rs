use super::{egui, ThermalApp, mlx};
use super::{SCALE_X_SPACE, IMAGE_X_SPACE};

mod image;
mod scale;

pub fn show(app: &mut ThermalApp, ui: &mut egui::Ui, ctx: &egui::Context) {
    image::update_image(app, ctx);

    ui.vertical_centered(
        |ui| {
            ui.horizontal_centered(
                |ui| {
                    scale::show_scale(app, ui);
                    image::show_image(app, ui);
                }
            );
    });
}

pub fn update_scale(app: &mut ThermalApp) {
    scale::update_scale(app);
}

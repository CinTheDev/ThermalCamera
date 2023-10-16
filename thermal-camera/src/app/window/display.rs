use super::{egui, ThermalApp, mlx};

pub mod image;
pub mod scale;

pub fn show(app: &mut ThermalApp, ui: &mut egui::Ui, ctx: &egui::Context) {
    app.update_image(ctx);

    ui.vertical_centered(
        |ui| {
            ui.horizontal_centered(
                |ui| {
                    app.show_scale(ui);
                    app.show_image(ui);
                }
            );
    });
}

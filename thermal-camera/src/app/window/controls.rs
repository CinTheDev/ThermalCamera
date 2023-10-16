use super::{egui, ThermalApp, mlx};

pub fn show(app: &mut ThermalApp, ui: &mut egui::Ui) {

}

fn button_freeze(app: &mut ThermalApp) {
    app.rx_active = !app.rx_active;
}

fn button_save(app: &mut ThermalApp) {
    app.save_image();
    println!("Image saved");
}

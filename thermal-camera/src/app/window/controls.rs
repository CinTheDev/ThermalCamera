use super::{egui, ThermalApp};

pub fn show(app: &mut ThermalApp, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        let button_freeze = ui.button("Freeze image");
        let button_save = ui.add_enabled(
            app.usb_detected,
            egui::Button::new("Save image")
        );

        if button_freeze.clicked() {
            on_button_freeze(app);
        }
        if button_save.clicked() {
            on_button_save(app);
        }
    });
}

fn on_button_freeze(app: &mut ThermalApp) {
    app.rx_active = !app.rx_active;
}

fn on_button_save(app: &mut ThermalApp) {
    app.save_image();
    println!("Image saved");
}

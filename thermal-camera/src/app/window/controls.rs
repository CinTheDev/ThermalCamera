use super::{egui, ThermalApp};
use super::CONTROLS_X_SPACE;

pub fn show(app: &mut ThermalApp, ui: &mut egui::Ui) {
    let width_allocate = CONTROLS_X_SPACE * app.window_size.x;
    let height_buttons = ui.available_height() / 2.0;
    let size_buttons = egui::Vec2::new(width_allocate, height_buttons);

    ui.vertical_centered(|ui| {
        //ui.set_min_width(width_allocate);

        let button_freeze = ui.add(
            egui::Button::new("Freeze image").min_size(size_buttons)
        );
        let button_save = ui.add_enabled(
            app.usb_detected,
            egui::Button::new("Save image").min_size(size_buttons)
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

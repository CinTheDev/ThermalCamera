use super::{egui, ThermalApp, mlx};

mod options;

pub fn show(app: &mut ThermalApp, ui: &mut egui::Ui) {
    let height_buttons = app.window_size.y / 3.0;
    let size_buttons = egui::Vec2::new(0.0, height_buttons);

    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
        let button_freeze = ui.add(
            egui::Button::new("Freeze image").min_size(size_buttons)
        );
        let button_save = ui.add_enabled(
            app.usb_detected,
            egui::Button::new("Save image").min_size(size_buttons)
        );
        let button_options = ui.add(
            egui::Button::new("Options").min_size(size_buttons)
        );

        if button_freeze.clicked() {
            on_button_freeze(app);
        }
        if button_save.clicked() {
            on_button_save(app);
        }
        if button_options.clicked() {
            on_button_options(app);
        }
    });

    if app.show_options {
        options::show(app, ui);
    }
}

fn on_button_freeze(app: &mut ThermalApp) {
    app.rx_active = !app.rx_active;
}

fn on_button_save(app: &mut ThermalApp) {
    app.save_image();
    println!("Image saved");
}

fn on_button_options(app: &mut ThermalApp) {
    app.show_options = true;
}

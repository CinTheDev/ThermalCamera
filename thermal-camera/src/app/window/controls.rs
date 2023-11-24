use super::{egui, ThermalApp, CONTROLS_X_SPACE};

pub fn show(app: &mut ThermalApp, ui: &mut egui::Ui) {
    const BUTTONS_COUNT: u32 = 3;
    let width = CONTROLS_X_SPACE * app.window_size.x;
    let spacing_y = ui.spacing().item_spacing.y;
    let height_buttons = ui.available_height() / BUTTONS_COUNT as f32 - spacing_y;
    let size_buttons = egui::Vec2::new(width, height_buttons);

    ui.vertical(|ui| {
        let button_freeze = ui.add_sized(
            size_buttons,
            egui::Button::new("Freeze image")
        );

        ui.add_enabled_ui(
            app.usb_detected,
            |ui| {
                let button_save = ui.add_sized(
                    size_buttons,
                    egui::Button::new("Save image")
                );

                if button_save.clicked() {
                    on_button_save(app);
                }
            }
        ).response;

        let button_options = ui.add_sized(
            size_buttons,
            egui::Button::new("Options")
        );

        if button_freeze.clicked() {
            on_button_freeze(app);
        }
        if button_options.clicked() {
            on_button_options(app);
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

fn on_button_options(app: &mut ThermalApp) {
    app.show_options = true;
}

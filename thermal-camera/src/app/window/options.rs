use super::{egui, ThermalApp, mlx};

const WINDOW_RELATIVE_SIZE: f32 = 0.9;

pub fn show(app: &mut ThermalApp, ui: &mut egui::Ui) {
    if ! app.show_options { return }

    let center = egui::Pos2::new(app.window_size.x / 2.0, app.window_size.y / 2.0);
    let size = app.window_size * WINDOW_RELATIVE_SIZE;
    let bounds = egui::Rect::from_center_size(center, size);

    ui.put(bounds, options_window::new());
}

struct options_window {

}

impl options_window {
    fn new() -> Self {
        return options_window {};
    }
}

impl egui::widgets::Widget for options_window {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        return ui.vertical(|ui| {
            ui.button("Test 1");
            ui.button("Test 2");
        }).response;
    }
}

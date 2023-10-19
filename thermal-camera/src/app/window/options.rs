use super::{egui, ThermalApp, mlx};

const WINDOW_RELATIVE_SIZE: f32 = 0.9;

pub fn show(app: &mut ThermalApp, ui: &mut egui::Ui) {
    if ! app.show_options { return }

    let center = egui::Pos2::new(app.window_size.x / 2.0, app.window_size.y / 2.0);
    let size = app.window_size * WINDOW_RELATIVE_SIZE;
    let bounds = egui::Rect::from_center_size(center, size);

    let mut option_ui = ui.child_ui(bounds, egui::Layout::top_down_justified(egui::Align::Min));
    draw_options(app, &mut option_ui);
}

fn draw_options(app: &mut ThermalApp, ui: &mut egui::Ui) {
    let elements_height = ui.available_height() / 2.0;
    let elements_size = egui::vec2(0.0, elements_height);

    ui.add(
        egui::Button::new("Test 1").min_size(elements_size)
    );
    ui.add(
        egui::Button::new("Test 2").min_size(elements_size)
    );
}

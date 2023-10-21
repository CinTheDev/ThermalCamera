use super::{egui, ThermalApp, mlx};

const WINDOW_RELATIVE_SIZE: f32 = 0.9;

pub fn show(app: &mut ThermalApp, ui: &mut egui::Ui) {
    if ! app.show_options { return }

    let center = egui::Pos2::new(app.window_size.x / 2.0, app.window_size.y / 2.0);
    let size = app.window_size * WINDOW_RELATIVE_SIZE;
    let bounds = egui::Rect::from_center_size(center, size);

    let mut option_ui = ui.child_ui(bounds, egui::Layout::top_down(egui::Align::Min));
    draw_options(app, &mut option_ui);
}

fn draw_options(app: &mut ThermalApp, ui: &mut egui::Ui) {
    let bg_color = egui::Color32::BLACK;
    let bg_painter = ui.painter();
    bg_painter.rect_filled(ui.max_rect(), 0.0, bg_color);

    let elements_height = ui.available_height() / 2.0;
    let elements_size = egui::vec2(0.0, elements_height);

    let btn_1 = ui.add(
        egui::Button::new("Make grayscale").min_size(elements_size)
    );
    let btn_2 = ui.add(
        egui::Button::new("Test 2").min_size(elements_size)
    );

    if btn_1.clicked() {
        app.options.color_type = mlx::ColorTypes::Gray;
        app.show_options = false;
        app.update_options();
        app.recolor_image(ui.ctx());
    }
    if btn_2.clicked() {
        app.show_options = false;
        app.update_options();
    }
}

use super::{egui, ThermalApp, mlx};

const WINDOW_RELATIVE_SIZE: f32 = 0.8;

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
    bg_painter.rect_filled(ui.max_rect().expand(10.0), 5.0, bg_color);

    const ROWS: u32 = 2;
    let elements_height = ui.available_height() / ROWS as f32;
    let element_standard_size = egui::vec2(ui.available_width(), elements_height);

    // Close menu button
    let btn_close = ui.add(
        egui::Button::new("Close").min_size(element_standard_size)
    );

    // Color options
    const OPTIONS_COLORING_COLUMNS: u32 = 3;
    let options_coloring_width = ui.available_width() / OPTIONS_COLORING_COLUMNS as f32;
    let options_coloring_size = egui::vec2(options_coloring_width, elements_height);

    ui.horizontal(|ui| {
        let btn_coloring_gray = ui.add(
            egui::Button::new("Grayscale").min_size(options_coloring_size)
        );
        let btn_coloring_cheap = ui.add(
            egui::Button::new("Cheap").min_size(options_coloring_size)
        );
        let btn_coloring_hue = ui.add(
            egui::Button::new("Hue").min_size(options_coloring_size)
        );

        if btn_coloring_gray.clicked() {
            on_btn_coloring(app, ui.ctx(), mlx::ColorTypes::Gray);
        }
        if btn_coloring_cheap.clicked() {
            on_btn_coloring(app, ui.ctx(), mlx::ColorTypes::Cheap);
        }
        if btn_coloring_hue.clicked() {
            on_btn_coloring(app, ui.ctx(), mlx::ColorTypes::Hue);
        }
    });

    if btn_close.clicked() {
        on_btn_close(app);
    }
}

fn on_btn_close(app: &mut ThermalApp) {
    app.show_options = false;
    app.update_options();
}

fn on_btn_coloring(app: &mut ThermalApp, ctx: &egui::Context, color_type: mlx::ColorTypes) {
    app.options.color_type = color_type;
    app.show_options = false;
    app.update_options();
    app.recolor_image(ctx);
}

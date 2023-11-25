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

// TODO: Mirror options in left-hand-mode in a sensical way
fn draw_options(app: &mut ThermalApp, ui: &mut egui::Ui) {
    let bg_color = egui::Color32::BLACK;
    let bg_painter = ui.painter();
    bg_painter.rect_filled(ui.max_rect().expand(10.0), 5.0, bg_color);

    const ROWS: u32 = 4;
    let spacing_y = ui.spacing().item_spacing.y;
    let elements_height = ui.available_height() / ROWS as f32 - spacing_y;
    let element_standard_size = egui::vec2(0.0, elements_height);
    let label_size = egui::vec2(100.0, elements_height / 2.0);

    ui.vertical_centered_justified(|ui| {
        // Close menu button
        handle_close_button(ui, app, element_standard_size);

        // Color options
        ui.horizontal(|ui| {
            draw_label_color(ui, &app, label_size);

            handle_options_color(ui, app, element_standard_size);
        });

        // Speed options
        ui.horizontal(|ui| {
            draw_label_speed(ui, element_standard_size);

            handle_options_speed(ui, app, element_standard_size);
        });

        // Handedness
        ui.horizontal(|ui| {
            draw_label_handedness(ui, element_standard_size);

            handle_options_handedness(ui, app, element_standard_size);
        });
    });
}

fn handle_close_button(ui: &mut egui::Ui, app: &mut ThermalApp, element_size: egui::Vec2) {
    let btn_close = ui.add_sized(
        element_size,
        egui::Button::new("Close")
    );

    if btn_close.clicked() {
        on_btn_close(app);
    }
}

fn draw_label_color(ui: &mut egui::Ui, app: &ThermalApp, element_size: egui::Vec2) {
    ui.allocate_ui(element_size,
        |ui| {
        ui.vertical(|ui| {
            ui.add_sized(
                element_size,
                egui::Label::new("Color Mode")
            );
    
            ui.add_sized(
                element_size,
                egui::Label::new("Test")
            );
        });
    });
}

fn handle_options_color(ui: &mut egui::Ui, app: &mut ThermalApp, element_size: egui::Vec2) {
    ui.columns(3, |col| {
        let btn_coloring_gray = col[0].add_sized(
            element_size,
            egui::Button::new("Grayscale")
        );
        let btn_coloring_cheap = col[1].add_sized(
            element_size,
            egui::Button::new("Cheap")
        );
        let btn_coloring_hue = col[2].add_sized(
            element_size,
            egui::Button::new("Hue")
        );

        if btn_coloring_gray.clicked() {
            on_btn_coloring(app, col[0].ctx(), mlx::ColorTypes::Gray);
        }
        if btn_coloring_cheap.clicked() {
            on_btn_coloring(app, col[1].ctx(), mlx::ColorTypes::Cheap);
        }
        if btn_coloring_hue.clicked() {
            on_btn_coloring(app, col[2].ctx(), mlx::ColorTypes::Hue);
        }
    });
}

fn draw_label_speed(ui: &mut egui::Ui, element_size: egui::Vec2) {
    ui.add_sized(
        element_size,
        egui::Label::new("Framerate")
    );
}

fn handle_options_speed(ui: &mut egui::Ui, app: &mut ThermalApp, element_size: egui::Vec2) {
    ui.columns(3, |col| {
        let btn_speed_low = col[0].add_sized(
            element_size,
            egui::Button::new("Low")
        );
        let btn_speed_medium = col[1].add_sized(
            element_size,
            egui::Button::new("Medium")
        );
        let btn_speed_high = col[2].add_sized(
            element_size,
            egui::Button::new("High")
        );

        if btn_speed_low.clicked() {
            on_btn_speed(app, mlx::Framerates::Two);
        }
        if btn_speed_medium.clicked() {
            on_btn_speed(app, mlx::Framerates::Eight);
        }
        if btn_speed_high.clicked() {
            on_btn_speed(app, mlx::Framerates::Thirtytwo);
        }
    });
}

fn draw_label_handedness(ui: &mut egui::Ui, element_size: egui::Vec2) {
    ui.add_sized(
        element_size,
        egui::Label::new("Left Hand Mode")
    );
}

fn handle_options_handedness(ui: &mut egui::Ui, app: &mut ThermalApp, element_size: egui::Vec2) {
    let btn_left_hand = ui.add_sized(
        element_size,
        egui::Button::new("Toggle")
    );

    if btn_left_hand.clicked() {
        on_toggle_hand(app);
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

fn on_btn_speed(app: &mut ThermalApp, framerate: mlx::Framerates) {
    app.options.framerate = framerate;
    mlx::set_framerate(framerate);
    
    app.show_options = false;
    app.update_options();
}

fn on_toggle_hand(app: &mut ThermalApp) {
    app.options.left_handed = !app.options.left_handed;

    app.show_options = false;
    app.update_options();
}

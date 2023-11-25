use super::{egui, ThermalApp, mlx};

const WINDOW_RELATIVE_SIZE: f32 = 0.8;
const LABEL_WIDTH: f32 = 100.0;
const LABEL_SELECTED_COL: egui::Color32 = egui::Color32::YELLOW;

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
    let label_size = egui::vec2(LABEL_WIDTH, elements_height / 2.0);

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
            draw_label_speed(ui, app, label_size);

            handle_options_speed(ui, app, element_standard_size);
        });

        // Handedness
        ui.horizontal(|ui| {
            draw_label_handedness(ui, app, label_size);

            handle_options_handedness(ui, app, element_standard_size);
        });
    });
}

fn draw_label(ui: &mut egui::Ui, label_size: egui::Vec2, label_text: &str, label_val: String) {
    let label_val_richtext = egui::RichText::new(label_val)
        .color(LABEL_SELECTED_COL);

    ui.vertical(|ui| {
        ui.add_sized(
            label_size,
            egui::Label::new(label_text)
        );

        ui.add_sized(
            label_size,
            egui::Label::new(label_val_richtext)
        );
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

fn draw_label_color(ui: &mut egui::Ui, app: &ThermalApp, label_size: egui::Vec2) {
    let label_text = "Color mode";
    let label_val = app.options.color_type.to_string();

    draw_label(ui, label_size, label_text, label_val);
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

fn draw_label_speed(ui: &mut egui::Ui, app: &ThermalApp, label_size: egui::Vec2) {
    let label_text = "Refresh Rate";
    let label_val = app.options.framerate.to_string() + " fps";

    draw_label(ui, label_size, label_text, label_val);
}

fn handle_options_speed(ui: &mut egui::Ui, app: &mut ThermalApp, element_size: egui::Vec2) {
    let is_on_lower_bound = app.options.framerate as u8 == mlx::Framerates::Half as u8;
    let is_on_upper_bound = app.options.framerate as u8 == mlx::Framerates::Sixtyfour as u8;

    ui.columns(2, |col| {
        let btn_speed_decrease = col[0].add_enabled_ui(
            !is_on_lower_bound,
            |ui| {
                ui.add_sized(
                    element_size,
                    egui::Button::new("Decrease")
                )
            }
        ).inner;

        let btn_speed_increase = col[1].add_enabled_ui(
            !is_on_upper_bound,
            |ui| {
                ui.add_sized(
                    element_size,
                    egui::Button::new("Increase")
                )
            }
        ).inner;

        if btn_speed_decrease.clicked() {

        }

        if btn_speed_increase.clicked() {

        }
    });
}

fn draw_label_handedness(ui: &mut egui::Ui, app: &ThermalApp, label_size: egui::Vec2) {
    let label_text = "Layout";
    let label_val = match app.options.left_handed {
        false => "Right handed".into(),
        true => "Left handed".into(),
    };

    draw_label(ui, label_size, label_text, label_val);
}

fn handle_options_handedness(ui: &mut egui::Ui, app: &mut ThermalApp, element_size: egui::Vec2) {
    let is_left_hand = app.options.left_handed;

    ui.columns(2, |col| {
        let btn_left_hand = col[0].add_enabled_ui(
            !is_left_hand,
            |ui| {
                ui.add_sized(
                    element_size,
                    egui::Button::new("Left hand")
                )
            }
        ).inner;
    
        let btn_right_hand = col[1].add_enabled_ui(
            is_left_hand,
            |ui| {
                ui.add_sized(
                    element_size,
                    egui::Button::new("Right hand")
                )
            }
        ).inner;
    
        if btn_left_hand.clicked() {
            on_btn_hand(app, true);
        }
        
        if btn_right_hand.clicked() {
            on_btn_hand(app, false);
        }
    });
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

fn on_btn_hand(app: &mut ThermalApp, left_hand: bool) {
    app.options.left_handed = left_hand;

    app.show_options = false;
    app.update_options();
}

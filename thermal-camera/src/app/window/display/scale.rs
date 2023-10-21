use super::{egui, ThermalApp, mlx};
use super::SCALE_X_SPACE;

pub fn show_scale(app: &mut ThermalApp, ui: &mut egui::Ui) {
    let width_allocate = SCALE_X_SPACE * app.window_size.x;

    let texture = app.scale.as_ref().unwrap();

    ui.vertical(
        |ui| {
            let string_min_temp = format!("{:.1} °C", app.scale_bound.0);
            let string_max_temp = format!("{:.1} °C", app.scale_bound.1);
            ui.label(string_max_temp);

            let height = ui.available_height() - 20.0;
            let width = width_allocate;
            let size = egui::Vec2::new(width, height);
            ui.image(texture, size);

            ui.label(string_min_temp);
        }
    );
}

pub fn update_scale(app: &mut ThermalApp) {
    let color_type = app.options.color_type;
    let gradient = mlx::get_scale(color_type);

    app.raw_scale.replace(gradient);

    let scale = egui::ColorImage::from_rgb(
        [mlx::GRADIENT_WIDTH, mlx::GRADIENT_HEIGHT],
        &gradient
    );

    app.scale.as_mut().unwrap().set(scale, app.picture_options);
}

pub fn init_scale(app: &mut ThermalApp, ctx: &egui::Context) {
    let raw_scale = app.raw_scale.get_or_insert_with(|| {
        [0x00; mlx::GRADIENT_COUNT * 3]
    });

    let img = egui::ColorImage::from_rgb(
        [mlx::GRADIENT_WIDTH, mlx::GRADIENT_HEIGHT],
        raw_scale
    );

    let texture = ctx.load_texture("Scale", img, app.picture_options);
    app.scale.replace(texture);
}

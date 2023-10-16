use super::{egui, ThermalApp, mlx};

pub fn show_scale(app: &mut ThermalApp, ui: &mut egui::Ui) {
    let texture: &egui::TextureHandle = app.scale.get_or_insert_with(|| {
        let raw_scale = app.raw_scale.get_or_insert_with(|| {
            [0x00; mlx::GRADIENT_COUNT * 3]
        });

        let img = egui::ColorImage::from_rgb(
            [mlx::GRADIENT_WIDTH, mlx::GRADIENT_HEIGHT],
            raw_scale
        );

        ui.ctx().load_texture("Scale", img, app.picture_options)
    });

    ui.vertical(
        |ui| {
            let string_min_temp = format!("{:.1} °C", app.scale_bound.0);
            let string_max_temp = format!("{:.1} °C", app.scale_bound.1);
            ui.label(string_max_temp);

            let height = ui.available_height() - 20.0;
            let width = height * (mlx::GRADIENT_WIDTH as f32 / mlx::GRADIENT_HEIGHT as f32);
            let size = egui::Vec2 {x: width, y: height};
            ui.image(texture, size);

            ui.label(string_min_temp);
        }
    );
}

pub fn update_scale(app: &mut ThermalApp) {
    let color_type = app.options.color_type;
    let gradient = mlx::get_scale(color_type);

    app.raw_scale.replace(gradient);
}

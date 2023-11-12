use super::{egui, ThermalApp, mlx};
use super::IMAGE_X_SPACE;

pub fn show_image(app: &mut ThermalApp, ui: &mut egui::Ui) {
    let texture = app.picture.as_ref().unwrap();

    let width = IMAGE_X_SPACE * app.window_size.x;
    let height = width * (mlx::PIXELS_HEIGHT as f32 / mlx::PIXELS_WIDTH as f32);
    let size = egui::Vec2::new(width, height);

    let response = ui.image(texture, size);
    check_clicked(app, ui, response);
}

pub fn update_image(app: &mut ThermalApp, ctx: &egui::Context) {
    // Don't update image if not supposed to
    if !app.rx_active { return }

    let rx = app.get_thread_receiver(ctx);

    let rx_img = rx.try_recv();

    if rx_img.is_ok() {
        let img_response = rx_img.unwrap();

        if img_response.is_err() {
            app.last_read = img_response;
            return;
        }

        let img_read = img_response.unwrap();
        
        let img = egui::ColorImage::from_rgb(
            [mlx::PIXELS_WIDTH, mlx::PIXELS_HEIGHT],
            &img_read.pixels
        );

        app.picture.as_mut().unwrap().set(img, app.picture_options);
        
        app.scale_bound = (img_read.temperature_read.min_temp, img_read.temperature_read.max_temp);

        app.last_read = Ok(img_read);
    }
}

pub fn init_image_texture(app: &mut ThermalApp, ctx: &egui::Context) {
    let raw_img = [0x00; mlx::PIXEL_COUNT * 3];

    let img = egui::ColorImage::from_rgb(
        [mlx::PIXELS_WIDTH, mlx::PIXELS_HEIGHT],
        &raw_img
    );

    let texture = ctx.load_texture("Picture", img, app.picture_options);
    app.picture.replace(texture);
}

pub fn check_clicked(app: &mut ThermalApp, ui: &mut egui::Ui, response: egui::Response) {
    // TODO: Only when dragging the mouse this is registered
    // Could be the display, but should be investigated
    let mut left_mouse_down = false;
    ui.input(|i| left_mouse_down = i.pointer.button_down(egui::PointerButton::Primary));
    if ! left_mouse_down {
        return;
    }

    let pos_option = response.hover_pos();
    if pos_option.is_none() {
        return;
    }

    // Calculate position inside image
    let pos = pos_option.unwrap();

    let rect_image = response.rect;

    let uv = egui::Pos2::new(
        (pos.x - rect_image.left()) / rect_image.right(),
        (pos.y - rect_image.top()) / rect_image.bottom()
    );

    let img_coord: (usize, usize) = (
        (uv.x * mlx::PIXELS_WIDTH as f32).floor().min(mlx::PIXELS_WIDTH as f32).max(0.0) as usize,
        (uv.y * mlx::PIXELS_HEIGHT as f32).floor().min(mlx::PIXELS_HEIGHT as f32).max(0.0) as usize,
    );

    let index = img_coord.0 + img_coord.1 * mlx::PIXELS_WIDTH;

    let mut temperature = 0.0;

    if app.last_read.is_ok() {
        temperature = app.last_read.as_ref().unwrap().temperature_read.temperature_grid[index];
    }

    let temp_string = format!("{:.1} °C", temperature);

    let bg_col = egui::Color32::BLACK;
    let txt_col = egui::Color32::WHITE;
    
    // Draw
    let painter = ui.painter();
    let txt_galley = painter.layout_no_wrap(temp_string, egui::FontId::default(), txt_col);
    let bg_rect = txt_galley.rect
        .translate(pos.to_vec2())
        .expand(5.0);

    painter.rect_filled(bg_rect, egui::Rounding::none(), bg_col);
    painter.galley(pos, txt_galley);
}

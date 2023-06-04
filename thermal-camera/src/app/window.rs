use eframe::egui;
use super::mlx;
use std::thread;
use std::sync::mpsc;

pub use super::Opt;

pub fn open_window(args: Opt) {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Thermal Camera",
        native_options,
        Box::new(|cc| Box::new(ThermalApp::new(cc, args))),
    )
    .unwrap();
}

#[derive(Default)]
struct ThermalApp {
    options: Opt,
    picture: Option<egui::TextureHandle>,
    image_rx: Option<mpsc::Receiver<[u8; mlx::PIXEL_COUNT * 3]>>,
    rx_active: bool,
}

impl ThermalApp {
    fn new(_cc: &eframe::CreationContext<'_>, args: Opt) -> Self {
        Self {
            options: args,
            rx_active: true,
            ..Default::default()
        }
    }

    fn get_thread_receiver(&mut self, ctx: &egui::Context) -> &mut mpsc::Receiver<[u8; mlx::PIXEL_COUNT * 3]> {
        self.image_rx.get_or_insert_with(|| {
            let (tx, rx) = mpsc::channel();
            let ctx_clone = ctx.clone();
            let args_clone = self.options.clone();

            thread::spawn(move || ThermalApp::continuuos_read(args_clone, ctx_clone, tx));

            return rx;
        })
    }

    fn continuuos_read(args: Opt, ctx: egui::Context, tx: mpsc::Sender<[u8; mlx::PIXEL_COUNT * 3]>) -> ! {
        loop {
            let img = mlx::take_image(&args);
            tx.send(img).unwrap();
            ctx.request_repaint();
        }
    }

    fn show_image(&mut self, ui: &mut egui::Ui) {
        let texture: &egui::TextureHandle = self.picture.get_or_insert_with(|| {
            let img = egui::ColorImage::from_rgb(
                [mlx::PIXELS_WIDTH, mlx::PIXELS_HEIGHT],
                &[0x00; mlx::PIXEL_COUNT * 3]
            );

            ui.ctx()
                .load_texture("Picture", img, Default::default())
        });

        ui.painter().image(
            texture.id(),
            ui.available_rect_before_wrap(),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE
        );
    }
    
    fn update_image(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Don't update image if not supposed to
        if !self.rx_active { return }

        let rx = self.get_thread_receiver(ctx);

        let rx_img = rx.try_recv();

        if rx_img.is_ok() {
            let img = egui::ColorImage::from_rgb(
                [mlx::PIXELS_WIDTH, mlx::PIXELS_HEIGHT],
                &rx_img.unwrap()
            );

            self.picture.replace(ui.ctx().load_texture("Picture", img, Default::default()));
        }
    }

    fn save_image(&mut self) {
        if self.picture.is_none() { return }

        let pic = self.picture.as_ref().unwrap();

        // TODO: Read and save picture
    }
}

impl eframe::App for ThermalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: format this in a more sensical way
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                if ui.button("Freeze image").clicked() {
                    self.rx_active = !self.rx_active;
                }
            });

            self.update_image(ctx, ui);

            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center).with_main_justify(true), |ui| {
                self.show_image(ui);
            });
        });
    }
}

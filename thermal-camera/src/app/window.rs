use eframe::egui;
pub use super::Opt;

pub fn open_window(args: Opt) {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Thermal Camera",
        native_options,
        Box::new(|cc| Box::new(ThermalApp::new(cc))),
    )
    .unwrap();
}

#[derive(Default)]
struct ThermalApp {

}

impl ThermalApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        return Self::default();
    }
}

impl eframe::App for ThermalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    }
}

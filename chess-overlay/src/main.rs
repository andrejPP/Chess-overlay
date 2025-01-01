mod gui;
mod capture;
use gui::ChessOverlayGUI;

fn main() -> eframe::Result {
    env_logger::init();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 600.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Chess overlay",
        native_options,
        Box::new(|cc| Ok(Box::new(ChessOverlayGUI::new(cc)))),
    )
}

mod app;
mod services;
mod ui;

use app::SpotifyApp;

fn main() -> Result<(), eframe::Error> { // TODO convert errors to a custom error type
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Eric's Personal Work Environment", // FUTURE config this title to be dynamic based on user
        options,
        Box::new(|_cc| Ok(Box::new(SpotifyApp::default()))),
    )
}
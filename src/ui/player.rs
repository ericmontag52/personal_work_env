use crate::app::SpotifyApp;
use crate::app::PlayerCommand;

pub fn render_player_ui(ui: &mut egui::Ui, app: &mut SpotifyApp) {
    egui::CentralPanel::default().show(ui, |ui| {
        ui.heading("🎵 Spotify Remote");
        ui.separator();

        ui.add_space(15.0);
        ui.label(format!("Current Track:\n{}", app.current_track));
        ui.add_space(15.0);

        ui.horizontal(|ui| {
            if ui.button("⏮ Prev").clicked() {
                let _ = app.cmd_tx.send(PlayerCommand::PrevTrack);
            }
            if ui.button("▶ Play / Pause").clicked() {
                let _ = app.cmd_tx.send(PlayerCommand::TogglePlayPause);
            }
            if ui.button("⏭ Next").clicked() {
                let _ = app.cmd_tx.send(PlayerCommand::NextTrack);
            }
        });
    });
}
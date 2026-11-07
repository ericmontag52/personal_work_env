use crate::ui::player;
use std::sync::mpsc::{self, Receiver};
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub enum PlayerCommand {
    TogglePlayPause,
    NextTrack,
    PrevTrack,
}

pub struct SpotifyApp {
    pub current_track: String,
    receiver: Receiver<String>,
    pub cmd_tx: Sender<PlayerCommand>,
}

impl Default for SpotifyApp {
    fn default() -> Self {
        let (track_tx, receiver) = mpsc::channel();
        let (cmd_tx, cmd_rx) = mpsc::channel();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(crate::services::spotify::run_spotify_worker(track_tx, cmd_rx));
        });

        Self {
            current_track: "Initializing Spotify connection...".to_string(),
            receiver,
            cmd_tx,
        }
    }
}

impl eframe::App for SpotifyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Check if the background worker sent a new track title (non-blocking)
        if let Ok(track) = self.receiver.try_recv() {
            self.current_track = track;
        }

        // Request a continuous repaint so the UI updates when new data arrives
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));

        // Render modular UI components, passing `ui`
        player::render_player_ui(ui, self);
    }
}
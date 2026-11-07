use rspotify::prelude::*;
use rspotify::{AuthCodePkceSpotify, Credentials, OAuth};
use std::collections::HashSet;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use tokio::time::sleep;
use crate::app::PlayerCommand;

pub async fn run_spotify_worker(tx: Sender<String>, cmd_rx: Receiver<PlayerCommand>) {
    dotenvy::dotenv().ok();

    let creds = match Credentials::from_env() {
        Some(c) => c,
        None => {
            let _ = tx.send("Error: Missing Spotify Client ID/Secret in .env".to_string());
            return;
        }
    };

    let scopes = HashSet::from([
        "user-read-playback-state".to_string(),
        "user-read-currently-playing".to_string(),
        "user-modify-playback-state".to_string(),
    ]);

    let oauth = match OAuth::from_env(scopes) {
        Some(o) => o,
        None => {
            let _ = tx.send("Error: Missing redirect URI in .env".to_string());
            return;
        }
    };

    let mut spotify = AuthCodePkceSpotify::new(creds, oauth);

    let url = match spotify.get_authorize_url(None) {
        Ok(u) => u,
        Err(e) => {
            let _ = tx.send(format!("Auth URL Error: {e}"));
            return;
        }
    };

    println!("Open this URL in your browser to log into Spotify:\n{url}");
    
    if let Err(e) = spotify.prompt_for_token(&url).await {
        let _ = tx.send(format!("Authorization Failed: {e}"));
        return;
    }

    let _ = tx.send("Connected to Spotify!".to_string());

    loop {
        // 1. Process any UI button commands non-blockingly
        while let Ok(cmd) = cmd_rx.try_recv() {
            match cmd {
                PlayerCommand::TogglePlayPause => {
                    if let Ok(Some(playback)) = spotify.current_playback(None, None::<&[rspotify::model::AdditionalType]>).await {
                        if playback.is_playing {
                            let _ = spotify.pause_playback(None).await;
                        } else {
                            // Use resume_playback to unpause current playback
                            let _ = spotify.resume_playback(None, None).await;
                        }
                    }
                }
                PlayerCommand::NextTrack => {
                    let _ = spotify.next_track(None).await;
                }
                PlayerCommand::PrevTrack => {
                    let _ = spotify.previous_track(None).await;
                }
            }
        }

        // 2. Poll current playback status
        match spotify.current_playback(None, None::<&[rspotify::model::AdditionalType]>).await {
            Ok(Some(playback)) => {
                let mut track_info = None;

                if let Some(item) = playback.item {
                    match item {
                        rspotify::model::PlayableItem::Track(track) => {
                            if let Some(artist) = track.artists.first() {
                                track_info = Some(format!("{} - {}", artist.name, track.name));
                            }
                        }
                        rspotify::model::PlayableItem::Unknown(val) => {
                            if let (Some(name), Some(artists)) = (val.get("name").and_then(|v| v.as_str()), val.get("artists").and_then(|v| v.as_array())) {
                                if let Some(artist_obj) = artists.first() {
                                    if let Some(artist_name) = artist_obj.get("name").and_then(|v| v.as_str()) {
                                        track_info = Some(format!("{} - {}", artist_name, name));
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(info) = track_info {
                    let _ = tx.send(info);
                } else {
                    let _ = tx.send("Paused or Non-track Audio".to_string());
                }
            }
            Ok(None) => {
                let _ = tx.send("No active playback found".to_string());
            }
            Err(e) => {
                let _ = tx.send(format!("API Error: {e}"));
            }
        }

        sleep(Duration::from_secs(1)).await;
    }
}
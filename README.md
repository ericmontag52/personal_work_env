# Personal Work Environment

A small Rust desktop app for controlling Spotify playback from a lightweight local work environment. It displays the current track and provides basic controls to play/pause, skip forward, and go back.

## Features

- Shows the currently playing track in the UI
- Supports play/pause control
- Supports next-track and previous-track control
- Uses Spotify OAuth with environment-based configuration

## Requirements

- Rust and Cargo installed
- A Spotify developer app with client credentials
- A local redirect URI that matches the configured OAuth callback

## Spotify setup

1. Go to the Spotify Developer Dashboard.
2. Create a new app.
3. Copy the Client ID and Client Secret.
4. Set the redirect URI to something like:
   - `http://localhost:8888/callback`

## Environment variables

Create a `.env` file in the project root. The app uses the following variables:

```env
SPOTIFY_CLIENT_ID=your_spotify_client_id
SPOTIFY_CLIENT_SECRET=your_spotify_client_secret
SPOTIFY_REDIRECT_URI=http://localhost:8888/callback
```

A template is included in [.env.example](.env.example).

## Running the app

1. Copy the example file:

   ```bash
   cp .env.example .env
   ```

2. Replace the placeholder values with your Spotify app credentials.
3. Build and run:

   ```bash
   cargo run
   ```

4. The app will print an authorization URL in the terminal. Open it in your browser and complete the Spotify login flow.

## Notes

- The `.env` file is intentionally ignored by git via `.gitignore`.
- The app currently expects Spotify OAuth from the environment, so the variables above must be present before the app starts.
- This project is still a local utility and is intended to be used as a minimal personal work environment tool.


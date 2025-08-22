# NowPlaying Sync

This project allows you to capture the current music playing in your browser using a Chrome Extension, and send it to a local Rust server. The Rust backend saves the information as a JSON file and also as separate `.txt` files for title, artist, and duration. Additionally, the server exposes an endpoint to fetch the now playing information.

---

## Features

- Browser extension (Chrome) to detect the currently playing music.
- Sends song details (title, artist, duration) to a local Rust server.
- Rust server:
  - Saves the data to a JSON file (`nowplaying.json`).
  - Saves each field in a separate `.txt` file (`title.txt`, `artist.txt`, `duration.txt`).
  - Exposes the JSON file on `http://127.0.0.1:7539/nowplaying`.
  - Logs every incoming request to the console.

---

## Requirements

- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- Chrome or a Chromium-based browser

---

## Installation

### 1. Build and run the Rust server

```bash
cargo run
```

This will start the server at `http://127.0.0.1:7539`.

### 2. Install the Chrome Extension

1. Open Chrome and go to `chrome://extensions/`.
2. Enable **Developer mode** (top-right corner).
3. Click **Load unpacked** and select the extension folder of this project.
4. The extension will start running in the background.

---

## Usage

1. Play a song in your browser (e.g., YouTube, Spotify Web, SoundCloud).
2. The extension will detect the currently playing track and send it to the Rust server.
3. The server will:
   - Log the request in the console.
   - Save the song details into:
     - `nowplaying.json`
     - `title.txt`
     - `artist.txt`
     - `duration.txt`
4. You can fetch the JSON data directly at:
   ```
   http://127.0.0.1:7539/nowplaying
   ```

---

## Example JSON output

```json
{
  "title": "Song Title",
  "artist": "Artist Name",
  "duration": "3:45"
}
```

---

## License

This project is licensed under the terms specified in the [LICENSE](./LICENSE) file.

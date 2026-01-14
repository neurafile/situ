# Piano Player Example

This example demonstrates the Gadget VM's ability to control audio playback through WASM.

## Features

- Plays C4-C5 notes from the acoustic grand piano samples
- WASM manages which sounds are playing
- WASM handles stopping sounds
- Beautiful piano keyboard interface

## Setup

1. Build the project from the root directory:
   ```bash
   make all
   ```

2. Serve the example using a local web server (required due to CORS):
   ```bash
   # Using Python 3
   python3 -m http.server 8000
   
   # Or using Node.js (if you have http-server installed)
   npx http-server -p 8000
   ```

3. Open in browser:
   ```
   http://localhost:8000/gadget-vm/examples/piano-player.html
   ```

## How It Works

- The Rust VM (compiled to WASM) manages the state of playing sounds
- When you click a key, the WASM `play_sound()` method is called
- The WASM calls JavaScript functions (`playSound`/`stopSound`) to actually play/stop audio
- The WASM tracks which sounds are active and can stop them on demand

## Notes

- The example requires the WASM files to be built and copied to `js-bindings/wasm/`
- The example requires the JavaScript bindings to be built to `js-bindings/dist/`
- Audio files are loaded from the `acoustic_grand_piano-mp3/` directory

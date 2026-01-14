# Gadget VM Examples

This directory contains example implementations demonstrating the Gadget VM's capabilities.

## Examples

### Piano Player (`piano-player.html`)

This example demonstrates the Gadget VM's ability to control audio playback through WASM.

**Features:**
- Plays C4-C5 notes from the acoustic grand piano samples
- WASM manages which sounds are playing
- WASM handles stopping sounds
- Beautiful piano keyboard interface

**How It Works:**
- The Rust VM (compiled to WASM) manages the state of playing sounds
- When you click a key, the WASM `play_sound()` method is called
- The WASM calls JavaScript functions (`playSound`/`stopSound`) to actually play/stop audio
- The WASM tracks which sounds are active and can stop them on demand

### BabylonJS Example (`babylonjs-example.html`)

This example demonstrates the Gadget VM integrated with BabylonJS for 3D graphics rendering.

**Features:**
- 3D scene with interactive camera controls
- Integration with BabylonJS engine
- Parse and load gadget definitions
- Interactive gadget definition editor

**How It Works:**
- Creates a BabylonJS scene with a 3D box and ground plane
- Initializes the Gadget VM integration with the BabylonJS scene
- Allows users to edit and load gadget definitions
- The VM parses gadget definitions and integrates with the 3D scene

### ThreeJS Example (`threejs-example.html`)

This example demonstrates the Gadget VM integrated with ThreeJS for 3D graphics rendering.

**Features:**
- 3D scene with interactive camera controls (OrbitControls)
- Integration with ThreeJS renderer
- Parse and load gadget definitions
- Interactive gadget definition editor

**How It Works:**
- Creates a ThreeJS scene with a 3D box and ground plane
- Initializes the Gadget VM integration with the ThreeJS scene
- Allows users to edit and load gadget definitions
- The VM parses gadget definitions and integrates with the 3D scene

## Setup

1. Build the project from the root directory:
   ```bash
   make all
   ```

2. Serve the examples using a local web server (required due to CORS):
   ```bash
   # Using Python 3
   python3 -m http.server 8000
   
   # Or using Node.js (if you have http-server installed)
   npx http-server -p 8000
   ```

3. Open in browser:
   ```
   # Piano Player
   http://localhost:8000/gadget-vm/examples/piano-player.html
   
   # BabylonJS Example
   http://localhost:8000/gadget-vm/examples/babylonjs-example.html
   
   # ThreeJS Example
   http://localhost:8000/gadget-vm/examples/threejs-example.html
   ```

## Requirements

- The examples require the WASM files to be built and located in `js-bindings/wasm/`
- The examples require the JavaScript bindings to be built to `js-bindings/dist/`
- The piano player example requires audio files from the `acoustic_grand_piano-mp3/` directory
- BabylonJS and ThreeJS examples use CDN links for the respective libraries

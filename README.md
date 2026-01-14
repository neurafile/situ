# Situ - Gadget VM

A Virtual Machine for the **Gadget Language**, a domain-specific language (DSL) designed for creating interactive 3D objects and behaviors in virtual worlds. The VM compiles to WebAssembly (WASM) for high-performance execution in web-based 3D environments like BabylonJS and ThreeJS.

## Overview

Situ provides a complete toolchain for authoring, parsing, and executing gadget definitions in virtual worlds. Gadgets are interactive 3D objects with:

- **Colliders** - Physical shapes for collision detection (box, sphere, cylinder, capsule)
- **Listeners** - Event handlers that respond to collisions, triggers, or custom events
- **Actions** - Executable behaviors like animations, sounds, HTTP calls, and event emission

The system is built with a piano-first design philosophy, using structured keywords rather than free-form code, making it accessible for non-programmers while remaining powerful enough for complex interactions.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Gadget Language (DSL)                      │
│         collider • listener • action definitions              │
└───────────────────────┬───────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              ANTLR Grammar Parser (Rust)                     │
│              Generates parser from .g4 grammar                │
└───────────────────────┬───────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Rust VM (compiles to WASM)                      │
│         Event queue • Action execution • State mgmt          │
└───────────────────────┬───────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│         JavaScript/TypeScript Bindings (npm)                 │
│         WASM integration • BabylonJS • ThreeJS                │
└───────────────────────┬───────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Host Virtual World (Babylon/Three)              │
│         Physics • Collision Events • Action Execution        │
└─────────────────────────────────────────────────────────────┘
```

## Components

### 1. Grammar (`gadget-vm/grammar/`)

ANTLR grammar file defining the Gadget Language syntax:

- **Collider definitions**: Physical shapes with geometry
- **Listener definitions**: Event handlers with targets and actions
- **Action definitions**: Executable behaviors (animations, sounds, HTTP, events)

See `gadget-vm/grammar/GadgetLanguage.g4` for the complete grammar specification.

### 2. Rust VM (`gadget-vm/rust-vm/`)

Core Virtual Machine implementation:

- Parser generated from ANTLR grammar
- Event queue system for handling collisions and triggers
- Action execution engine
- Compiles to WASM for web deployment

**Key Features:**
- Sound management (play/stop with tracking)
- Minimal language processing
- WASM-optimized for performance

### 3. JavaScript Bindings (`gadget-vm/js-bindings/`)

npm package (`@gadget/vm`) providing:

- WASM module initialization and bindings
- TypeScript type definitions
- BabylonJS integration helpers
- ThreeJS integration helpers
- Audio management (HTML5 Audio API)

## Getting Started

### Prerequisites

- **Rust** toolchain with `wasm32-unknown-unknown` target
- **Java** (for ANTLR parser generation)
- **Node.js** and **npm** (for JavaScript bindings)
- **Make** (for build automation)

### Quick Setup

1. **Install Rust WASM target** (first time only):
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install wasm-bindgen-cli
   ```

2. **Build everything**:
   ```bash
   make all
   ```

   This will:
   - Generate the Rust parser from ANTLR grammar
   - Compile the Rust VM to WASM
   - Build the JavaScript/TypeScript bindings

3. **Try the example**:
   ```bash
   make example
   # Open gadget-vm/examples/piano-player.html in a browser
   ```

### Individual Build Steps

```bash
# Generate parser from grammar
make parser

# Compile to WASM
make wasm

# Build JavaScript bindings
make build-js
```

## Usage

### Install the Package

```bash
npm install @gadget/vm
```

### Basic Usage

```typescript
import { initVM, createVM, parse } from '@gadget/vm';

// Initialize WASM module
await initVM();

// Create a VM instance
const vm = await createVM();

// Parse gadget language source
const result = await parse(`
  collider box1 {
    shape box(1, 1, 1)
  }
  
  listener collision1 {
    on collision
    target "box1"
    action playSound
  }
  
  action playSound {
    playSound
    file = "sounds/click.mp3"
  }
`);
```

### BabylonJS Integration

```typescript
import { initBabylonIntegration, loadGadget } from '@gadget/vm/babylonjs';
import { Scene, Engine } from '@babylonjs/core';

const engine = new Engine(canvas);
const scene = new Scene(engine);

const integration = await initBabylonIntegration(scene);
await loadGadget(integration, gadgetSource);
```

### ThreeJS Integration

```typescript
import { initThreeJSIntegration, loadGadget } from '@gadget/vm/threejs';
import { Scene } from 'three';

const scene = new Scene();
const integration = await initThreeJSIntegration(scene);
await loadGadget(integration, gadgetSource);
```

### Sound Management

The VM includes built-in sound management:

```typescript
const vm = await createVM();

// Play a sound (returns sound ID)
const soundId = vm.play_sound('path/to/sound.mp3', 'noteName');

// Stop a specific sound
vm.stop_sound('noteName');

// Stop all sounds
vm.stop_all_sounds();

// Get list of active sounds
const active = vm.get_active_sounds();
```

## Gadget Language Syntax

### Collider Definition

```gadget
collider myBox {
  shape box(1, 2, 1)
}

collider mySphere {
  shape sphere(0.5)
}

collider myMesh {
  geometry "path/to/model.glb"
}
```

### Listener Definition

```gadget
listener onCollision {
  on collision
  target "myBox"
  action playSound
  action emitEvent
}
```

### Action Definition

```gadget
action playSound {
  playSound
  file = "sounds/click.mp3"
}

action runAnimation {
  runAnim
  name = "spin"
  target = "myBox"
}

action makeHttpCall {
  httpCall
  url = "https://api.example.com/event"
  method = "POST"
}
```

## Project Structure

```
situ/
├── Makefile                    # Root build automation
├── sequence.md                 # Architecture diagram (Mermaid)
├── README.md                   # This file
└── gadget-vm/
    ├── grammar/
    │   ├── GadgetLanguage.g4   # ANTLR grammar definition
    │   └── README.md
    ├── rust-vm/
    │   ├── Cargo.toml          # Rust project configuration
    │   ├── Makefile            # Rust build automation
    │   ├── build.sh            # WASM build script
    │   ├── src/
    │   │   ├── lib.rs          # Main WASM exports
    │   │   └── grammar/        # Generated parser (from ANTLR)
    │   └── README.md
    ├── js-bindings/
    │   ├── package.json        # npm package configuration
    │   ├── tsconfig.json       # TypeScript configuration
    │   ├── src/
    │   │   ├── index.ts        # Main exports
    │   │   ├── babylonjs.ts    # BabylonJS integration
    │   │   └── threejs.ts      # ThreeJS integration
    │   ├── wasm/               # Generated WASM files
    │   └── README.md
    ├── examples/
    │   ├── piano-player.html   # Interactive piano demo
    │   ├── acoustic_grand_piano-mp3/  # Sample audio files
    │   └── README.md
    ├── README.md               # Gadget VM documentation
    └── QUICKSTART.md           # Quick start guide
```

## Development Workflow

1. **Edit Grammar**: Modify `gadget-vm/grammar/GadgetLanguage.g4`
2. **Regenerate Parser**: Run `make parser` in `gadget-vm/rust-vm/`
3. **Implement VM Logic**: Add processing in `gadget-vm/rust-vm/src/`
4. **Build WASM**: Run `make wasm` in `gadget-vm/rust-vm/`
5. **Update Bindings**: Modify `gadget-vm/js-bindings/src/` as needed
6. **Build Package**: Run `npm run build` in `gadget-vm/js-bindings/`
7. **Test Integration**: Import package in BabylonJS/ThreeJS projects

## Current Status (v0)

The project is in active development. Current features:

✅ **Completed:**
- ANTLR grammar definition
- Rust parser generation
- WASM compilation
- JavaScript bindings
- Sound management (play/stop)
- Basic VM structure
- Piano player example

🚧 **In Progress:**
- Full parser integration
- IR (Protobuf) generation
- Event queue system
- Collision event handling
- Action execution
- BabylonJS mesh/collider integration
- ThreeJS object/collider integration

## Future Roadmap

- **v0**: Core VM, basic actions, collision handling
- **v1**: Multiplayer support, WebSocket integration
- **v2+**: LLM integration, TTS, advanced behaviors

See `sequence.md` for detailed architecture diagrams and planned features.

## Requirements

- Rust 1.70+ with `wasm32-unknown-unknown` target
- Java 8+ (for ANTLR)
- Node.js 18+ and npm
- Make

## License

MIT

## Contributing

This is an internal project. For questions or contributions, please contact the maintainers.
